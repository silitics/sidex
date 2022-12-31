//! Chumsky-based parser for Sidex definitions.

use chumsky::{prelude::*, Stream};
use sidex_ir as ir;

use crate::{
    ast,
    span::Span,
    tokens::{
        delimiters, diagnostic_from_error, keywords, punctuations, tokenize, Delimiter, DocKind,
        Punctuation, Token, TokenKind,
    },
};

/// Create a parser for a name.
fn name_parser() -> impl Parser<TokenKind, ast::Identifier, Error = Simple<TokenKind, Span>> + Clone
{
    select! { TokenKind::Identifier(identifier) => identifier }
        .map_with_span(|identifier, span| ast::Identifier(ast::Node::new(identifier, span)))
        .labelled("identifier")
}

/// Create a parser for parsing token streams.
fn token_stream_parser(
) -> impl Parser<TokenKind, Vec<Token>, Error = Simple<TokenKind, Span>> + Clone {
    let single_token =
        filter(TokenKind::is_not_delimiter).map_with_span(|kind, span| vec![Token { kind, span }]);

    recursive(|token_tree| {
        choice((
            single_token,
            // 🚧 TODO: Other delimiters should also be allowed.
            token_tree
                .repeated()
                .at_least(1)
                .flatten()
                .delimited_by(
                    just(delimiters::Parenthesis::OPEN),
                    just(delimiters::Parenthesis::CLOSE),
                )
                .map_with_span(|mut list: Vec<Token>, span: Span| {
                    let open = Token {
                        kind: delimiters::Parenthesis::OPEN,
                        span: span.clone().with_range(span.start()..span.start() + 1),
                    };
                    let close = Token {
                        kind: delimiters::Parenthesis::CLOSE,
                        span: span.clone().with_range(span.end() - 1..span.end()),
                    };
                    list.insert(0, open);
                    list.push(close);
                    list
                }),
        ))
    })
    .or_not()
    .map(|tokens| tokens.unwrap_or_default())
}

/// Create a parser for parsing attributes.
fn attr_parser() -> impl Parser<TokenKind, ast::Attr, Error = Simple<TokenKind, Span>> + Clone {
    recursive(|attr_parser| {
        choice((
            path_parser()
                .then_ignore(just(punctuations::Equals::ALONE))
                .then(attr_parser.clone())
                .map(|(path, value)| {
                    ast::Attr {
                        kind: ast::AttrKind::Assign(ast::AttrAssign {
                            path,
                            value: Box::new(value),
                        }),
                    }
                }),
            path_parser()
                .then(
                    attr_parser
                        .clone()
                        .separated_by(just(punctuations::Comma::ALONE))
                        .allow_trailing()
                        .delimited_by(
                            just(delimiters::Parenthesis::OPEN),
                            just(delimiters::Parenthesis::CLOSE),
                        ),
                )
                .map(|(path, elements)| {
                    ast::Attr {
                        kind: ast::AttrKind::List(ast::AttrList { path, elements }),
                    }
                }),
            path_parser()
                .then(
                    path_parser()
                        .map(|path| {
                            ast::Attr {
                                kind: ast::AttrKind::Path(path),
                            }
                        })
                        .repeated()
                        .at_least(1)
                        .or_not(),
                )
                .map(|(path, args)| {
                    if let Some(elements) = args {
                        ast::Attr {
                            kind: ast::AttrKind::List(ast::AttrList { path, elements }),
                        }
                    } else {
                        ast::Attr {
                            kind: ast::AttrKind::Path(path),
                        }
                    }
                }),
            token_stream_parser().map(|stream| {
                ast::Attr {
                    kind: ast::AttrKind::Tokens(ast::TokenStream(stream)),
                }
            }),
        ))
    })
}

fn attrs_parser() -> impl Parser<TokenKind, Vec<ast::Attr>, Error = Simple<TokenKind, Span>> + Clone
{
    let attr = just(punctuations::Hash::ALONE).ignore_then(attr_parser().delimited_by(
        just(delimiters::Bracket::OPEN),
        just(delimiters::Bracket::CLOSE),
    ));

    attr.repeated()
}

fn double_colon(
) -> impl Parser<TokenKind, (TokenKind, TokenKind), Error = Simple<TokenKind, Span>> + Clone {
    just(punctuations::Colon::COMPOSED)
        .then(just(punctuations::Colon::ALONE).or(just(punctuations::Colon::COMPOSED)))
}

fn arrow() -> impl Parser<TokenKind, (TokenKind, TokenKind), Error = Simple<TokenKind, Span>> + Clone
{
    just(punctuations::Minus::COMPOSED).then(just(punctuations::AngleClose::ALONE))
}

fn path_parser() -> impl Parser<TokenKind, ast::Path, Error = Simple<TokenKind, Span>> + Clone {
    double_colon()
        .or_not()
        .then(name_parser().then_ignore(double_colon()).repeated())
        .then(name_parser())
        .map(|((root, mut segments), last)| {
            segments.push(last);
            ast::Path {
                segments,
                is_absolute: root.is_some(),
            }
        })
}

fn type_expr_parser(
) -> impl Parser<TokenKind, ast::TypeExpr, Error = Simple<TokenKind, Span>> + Clone {
    recursive(|type_expr| {
        let container = type_expr
            .clone()
            .then(
                just(punctuations::Colon::ALONE)
                    .ignore_then(type_expr.clone())
                    .or_not(),
            )
            .map(|(left, right)| {
                match right {
                    Some(right) => {
                        ast::TypeExpr::Map(ast::MapTypeExpr {
                            key: Box::new(left),
                            value: Box::new(right),
                        })
                    }
                    None => {
                        ast::TypeExpr::Sequence(ast::SequenceTypeExpr {
                            element: Box::new(left),
                        })
                    }
                }
            })
            .delimited_by(
                just(delimiters::Bracket::OPEN),
                just(delimiters::Bracket::CLOSE),
            );
        let instance = path_parser()
            .then(
                type_expr
                    .separated_by(just(punctuations::Comma::ALONE))
                    .allow_trailing()
                    .delimited_by(
                        just(punctuations::AngleOpen::ALONE),
                        just(punctuations::AngleClose::ALONE)
                            .or(just(punctuations::AngleClose::COMPOSED)),
                    )
                    .or_not(),
            )
            .map(|(path, subst)| {
                ast::TypeExpr::Instance(ast::InstanceTypeExpr {
                    path,
                    subst: subst.unwrap_or_default(),
                })
            });
        choice((instance, container))
    })
}

fn type_vars_parser(
) -> impl Parser<TokenKind, Vec<ast::TypeVar>, Error = Simple<TokenKind, Span>> + Clone {
    name_parser()
        .separated_by(just(punctuations::Comma::ALONE))
        .allow_trailing()
        .delimited_by(
            just(punctuations::AngleOpen::ALONE),
            just(punctuations::AngleClose::ALONE),
        )
        .map(|vars| vars.into_iter().map(|name| ast::TypeVar { name }).collect())
        .or_not()
        .map(|vars| vars.unwrap_or_default())
}

fn docs_parser(
    expected: DocKind,
) -> impl Parser<TokenKind, ast::Docs, Error = Simple<TokenKind, Span>> + Clone {
    select! { TokenKind::Doc { doc, kind } if kind == expected => doc }
        .map_with_span(|doc, span| ast::Node::new(doc, span))
        .repeated()
        .map(|docs| ast::Docs(docs))
}

fn field_parser() -> impl Parser<TokenKind, ast::Field, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then(name_parser())
        .then(
            just(punctuations::QuestionMark::ALONE)
                .or(just(punctuations::QuestionMark::COMPOSED))
                .or_not(),
        )
        .then_ignore(just(punctuations::Colon::ALONE))
        .then(type_expr_parser())
        .map(|((((docs, attrs), name), optional), typ)| {
            ast::Field {
                name,
                docs,
                attrs,
                typ,
                is_optional: optional.is_some(),
            }
        })
}

fn variant_parser() -> impl Parser<TokenKind, ast::Variant, Error = Simple<TokenKind, Span>> + Clone
{
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then(name_parser())
        .then(
            just(punctuations::Colon::ALONE)
                .ignore_then(type_expr_parser())
                .or_not(),
        )
        .map(|(((docs, attrs), name), typ)| {
            ast::Variant {
                name,
                docs,
                attrs,
                typ,
            }
        })
}

fn def_with_inner_parser(
    keyword: TokenKind,
    inner: impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone,
) -> impl Parser<
    TokenKind,
    ((ast::Identifier, Vec<ast::TypeVar>), ast::DefKind),
    Error = Simple<TokenKind, Span>,
> + Clone {
    just(keyword)
        .ignore_then(name_parser())
        .then(type_vars_parser())
        .then(inner.delimited_by(
            just(delimiters::Brace::OPEN),
            just(delimiters::Brace::CLOSE),
        ))
}

fn record_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    field_parser()
        .separated_by(just(punctuations::Comma::ALONE))
        .allow_trailing()
        .map(|fields| ast::DefKind::RecordType(ast::RecordTypeDef { fields }))
}

fn variant_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    variant_parser()
        .separated_by(just(punctuations::Comma::ALONE))
        .allow_trailing()
        .map(|variants| ast::DefKind::VariantType(ast::VariantTypeDef { variants }))
}

fn method_param_parser(
) -> impl Parser<TokenKind, ast::MethodParam, Error = Simple<TokenKind, Span>> + Clone {
    name_parser()
        .then(
            just(punctuations::QuestionMark::ALONE)
                .or(just(punctuations::QuestionMark::COMPOSED))
                .or_not(),
        )
        .then_ignore(just(punctuations::Colon::ALONE))
        .then(type_expr_parser())
        .map(|((name, optional), typ)| {
            ast::MethodParam {
                name,
                typ,
                is_optional: optional.is_some(),
            }
        })
}

fn method_parser() -> impl Parser<TokenKind, ast::Method, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then_ignore(just(keywords::FUN))
        .then(name_parser())
        .then(
            method_param_parser()
                .separated_by(just(punctuations::Comma::ALONE))
                .allow_trailing()
                .delimited_by(
                    just(delimiters::Parenthesis::OPEN),
                    just(delimiters::Parenthesis::CLOSE),
                ),
        )
        .then(arrow().ignore_then(type_expr_parser()).or_not())
        .map(|((((docs, attrs), name), params), returns)| {
            ast::Method {
                name,
                docs,
                attrs,
                params,
                returns,
            }
        })
}

fn service_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    method_parser()
        .repeated()
        .map(|functions| ast::DefKind::Service(ast::ServiceDef { methods: functions }))
}

fn item_parser() -> impl Parser<TokenKind, ast::Item, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then(choice((
            def_with_inner_parser(keywords::RECORD, record_def_inner_parser()),
            def_with_inner_parser(keywords::VARIANT, variant_def_inner_parser()),
            def_with_inner_parser(keywords::SERVICE, service_def_inner_parser()),
            just(keywords::OPAQUE)
                .ignore_then(name_parser())
                .then(type_vars_parser())
                .map(|(name, vars)| {
                    (
                        (name, vars),
                        ast::DefKind::OpaqueType(ast::OpaqueTypeDef {}),
                    )
                }),
            just(keywords::DERIVED)
                .ignore_then(name_parser())
                .then(type_vars_parser())
                .map(|(name, vars)| {
                    (
                        (name, vars),
                        ast::DefKind::DerivedType(ast::DerivedTypeDef {}),
                    )
                }),
            just(keywords::ALIAS)
                .ignore_then(name_parser())
                .then(type_vars_parser())
                .then_ignore(just(punctuations::Colon::ALONE))
                .then(
                    type_expr_parser()
                        .map(|aliased| ast::DefKind::Alias(ast::AliasDef { aliased })),
                ),
            just(keywords::WRAPPER)
                .ignore_then(name_parser())
                .then(type_vars_parser())
                .then_ignore(just(punctuations::Colon::ALONE))
                .then(
                    type_expr_parser()
                        .map(|wrapped| ast::DefKind::WrapperType(ast::WrapperTypeDef { wrapped })),
                ),
        )))
        .map(|((docs, attrs), ((name, vars), kind))| {
            ast::Item::Def(ast::Def {
                name,
                docs,
                vars,
                attrs,
                kind,
            })
        })
}

fn import_parser() -> impl Parser<TokenKind, ast::Item, Error = Simple<TokenKind, Span>> + Clone {
    just(keywords::IMPORT)
        .ignore_then(recursive(|import_tree| {
            choice((
                just(punctuations::Asterisk::ALONE)
                    .or(just(punctuations::Asterisk::COMPOSED))
                    .map(|_| ast::ImportTree::Wildcard),
                double_colon()
                    .or_not()
                    .then(
                        name_parser()
                            .then_ignore(double_colon())
                            .repeated()
                            .at_least(1),
                    )
                    .then(choice((
                        import_tree
                            .separated_by(just(punctuations::Comma::ALONE))
                            .allow_trailing()
                            .delimited_by(
                                just(delimiters::Brace::OPEN),
                                just(delimiters::Brace::CLOSE),
                            ),
                        just(punctuations::Asterisk::ALONE)
                            .map(|_| vec![ast::ImportTree::Wildcard]),
                    )))
                    .map(|((root, prefix), trees)| {
                        let path = ast::Path {
                            segments: prefix,
                            is_absolute: root.is_some(),
                        };
                        ast::ImportTree::Group { path, trees }
                    }),
                path_parser().map(ast::ImportTree::Path),
            ))
        }))
        .map(|tree| ast::Item::Import(ast::Import { tree }))
}

fn schema_parser() -> impl Parser<TokenKind, ast::Schema, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Inline)
        .then(choice((item_parser(), import_parser())).repeated())
        .then_ignore(end())
        .map(|(docs, items)| {
            ast::Schema {
                docs,
                attrs: Vec::new(), // 🚧 TODO: Parse attrs from schema.
                items,
            }
        })
}

/// Internal parsing function.
fn parse_schema(source: &ir::Source, tokens: Vec<Token>) -> Option<ast::Schema> {
    let stream = Stream::from_iter(
        source.end().into(),
        tokens
            .into_iter()
            .filter(|token| !matches!(token.kind, TokenKind::Comment { .. }))
            .map(|token| (token.kind, token.span)),
    );

    let (module, errors) = schema_parser().parse_recovery(stream);

    for error in errors {
        diagnostic_from_error(error.map(|t| t.to_string())).emit();
    }

    module
}

/// Parse a schema.
pub fn parse(source: &ir::Source) -> Option<ast::Schema> {
    tokenize(source).and_then(|tokens| parse_schema(source, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_test_from_file {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let mut storage = ir::SourceStorage::new();
                let id = storage.insert(include_str!($path).to_owned(), None);
                let result = parse(&storage[id]);
                insta::assert_debug_snapshot!(stringify!($name), result);
            }
        };
    }

    make_test_from_file!(
        test_todo_list_api_manager,
        "../../../examples/todo-list/api/schemas/manager.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_ids,
        "../../../examples/todo-list/data/schemas/ids.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_person,
        "../../../examples/todo-list/data/schemas/person.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_task,
        "../../../examples/todo-list/data/schemas/task.sidex"
    );
}
