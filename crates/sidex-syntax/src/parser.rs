//! Chumsky-based parser for Sidex definitions.

use chumsky::{prelude::*, Stream};

use crate::{
    ast,
    errors::SyntaxError,
    reporting,
    source::{Source, SourceStorage, Span},
    tokens::{
        delimiters, keywords, punctuations, tokenize, Delimiter, DocKind, Punctuation, Token,
        TokenKind,
    },
};

/// Create a parser for a name.
fn name_parser() -> impl Parser<TokenKind, ast::Name, Error = Simple<TokenKind, Span>> + Clone {
    select! { TokenKind::Identifier(identifier) => identifier }
        .map_with_span(|identifier, span| ast::Name(ast::Node::new(identifier, span)))
        .labelled("identifier")
}

/// Create a parser for parsing token streams.
fn token_stream_parser(
) -> impl Parser<TokenKind, ast::TokenStream, Error = Simple<TokenKind, Span>> + Clone {
    let token_list = filter(TokenKind::is_not_delimiter)
        .map_with_span(|kind, span| Token { kind, span })
        .repeated()
        .at_least(1);

    recursive(|token_tree| {
        choice((
            token_list,
            // TODO: Other delimiters should also be allowed.
            token_tree
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
        .repeated()
        .at_least(1)
        .flatten()
    })
    .or_not()
    .map(|tokens| ast::TokenStream(tokens.unwrap_or_default()))
}

/// Create a parser for parsing attributes.
fn attrs_parser() -> impl Parser<TokenKind, Vec<ast::Attr>, Error = Simple<TokenKind, Span>> + Clone
{
    let attr = just(punctuations::Hash::ALONE)
        .ignore_then(
            name_parser()
                .then(
                    token_stream_parser()
                        .delimited_by(
                            just(delimiters::Parenthesis::OPEN),
                            just(delimiters::Parenthesis::CLOSE),
                        )
                        .or_not(),
                )
                .delimited_by(
                    just(delimiters::Bracket::OPEN),
                    just(delimiters::Bracket::CLOSE),
                ),
        )
        .map(|(name, args)| ast::Attr { name, args });

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
            .map(|(left, right)| match right {
                Some(right) => ast::TypeExpr::Map(ast::MapTypeExpr {
                    key: Box::new(left),
                    value: Box::new(right),
                }),
                None => ast::TypeExpr::Sequence(ast::SequenceTypeExpr {
                    element: Box::new(left),
                }),
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
                        just(punctuations::AngleClose::ALONE),
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

fn struct_field_parser(
) -> impl Parser<TokenKind, ast::StructField, Error = Simple<TokenKind, Span>> + Clone {
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
        .map(
            |((((docs, attrs), name), optional), typ)| ast::StructField {
                name,
                docs,
                attrs,
                typ,
                is_optional: optional.is_some(),
            },
        )
}

fn enum_variant_parser(
) -> impl Parser<TokenKind, ast::EnumVariant, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then(name_parser())
        .then(
            just(punctuations::Colon::ALONE)
                .ignore_then(type_expr_parser())
                .or_not(),
        )
        .map(|(((docs, attrs), name), typ)| ast::EnumVariant {
            name,
            docs,
            attrs,
            typ,
        })
}

fn def_with_inner_parser(
    keyword: TokenKind,
    inner: impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone,
) -> impl Parser<
    TokenKind,
    ((ast::Name, Vec<ast::TypeVar>), ast::DefKind),
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

fn struct_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    struct_field_parser()
        .separated_by(just(punctuations::Comma::ALONE))
        .allow_trailing()
        .map(|fields| ast::DefKind::Struct(ast::StructDef { fields }))
}

fn enum_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    enum_variant_parser()
        .separated_by(just(punctuations::Comma::ALONE))
        .allow_trailing()
        .map(|variants| ast::DefKind::Enum(ast::EnumDef { variants }))
}

fn function_param_parser(
) -> impl Parser<TokenKind, ast::FunctionParam, Error = Simple<TokenKind, Span>> + Clone {
    name_parser()
        .then(
            just(punctuations::QuestionMark::ALONE)
                .or(just(punctuations::QuestionMark::COMPOSED))
                .or_not(),
        )
        .then_ignore(just(punctuations::Colon::ALONE))
        .then(type_expr_parser())
        .map(|((name, optional), typ)| ast::FunctionParam {
            name,
            typ,
            is_optional: optional.is_some(),
        })
}

fn function_parser(
) -> impl Parser<TokenKind, ast::Function, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then_ignore(just(keywords::FUN))
        .then(name_parser())
        .then(
            function_param_parser()
                .separated_by(just(punctuations::Comma::ALONE))
                .allow_trailing()
                .delimited_by(
                    just(delimiters::Parenthesis::OPEN),
                    just(delimiters::Parenthesis::CLOSE),
                ),
        )
        .then_ignore(arrow())
        .then(type_expr_parser())
        .map(|((((docs, attrs), name), params), returns)| ast::Function {
            name,
            docs,
            attrs,
            params,
            returns,
        })
}

fn service_def_inner_parser(
) -> impl Parser<TokenKind, ast::DefKind, Error = Simple<TokenKind, Span>> + Clone {
    function_parser()
        .repeated()
        .map(|functions| ast::DefKind::Service(ast::ServiceDef { functions }))
}

fn item_parser() -> impl Parser<TokenKind, ast::Item, Error = Simple<TokenKind, Span>> + Clone {
    docs_parser(DocKind::Preceding)
        .then(attrs_parser())
        .then(choice((
            def_with_inner_parser(keywords::STRUCT, struct_def_inner_parser()),
            def_with_inner_parser(keywords::ENUM, enum_def_inner_parser()),
            def_with_inner_parser(keywords::SERVICE, service_def_inner_parser()),
            just(keywords::OPAQUE)
                .ignore_then(name_parser())
                .map(|name| {
                    (
                        (name, Default::default()),
                        ast::DefKind::Opaque(ast::OpaqueDef {}),
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
                    .then(
                        import_tree
                            .separated_by(just(punctuations::Comma::ALONE))
                            .allow_trailing()
                            .delimited_by(
                                just(delimiters::Brace::OPEN),
                                just(delimiters::Brace::CLOSE),
                            ),
                    )
                    .map(|((root, prefix), trees)| {
                        let path = ast::Path {
                            segments: prefix,
                            is_absolute: root.is_some(),
                        };
                        ast::ImportTree::From { path, trees }
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
        .map(|(docs, items)| ast::Schema { docs, items })
}

/// Internal parsing function.
fn parse_schema(source: &Source, tokens: Vec<Token>) -> (Option<ast::Schema>, Vec<SyntaxError>) {
    let stream = Stream::from_iter(
        source.end(),
        tokens
            .into_iter()
            .filter(|token| !matches!(token.kind, TokenKind::Comment { .. }))
            .map(|token| (token.kind, token.span)),
    );

    let (module, errors) = schema_parser().parse_recovery(stream);

    (
        module,
        errors
            .into_iter()
            .map(|error| SyntaxError(error.map(|t| t.to_string())))
            .collect(),
    )
}

/// Parse a schema.
pub fn parse(storage: &SourceStorage, source: &Source) -> Option<ast::Schema> {
    let (tokens, lexer_errors) = tokenize(source);

    let (schema, parse_errors) = match tokens {
        Some(tokens) => parse_schema(source, tokens),
        None => (None, Default::default()),
    };

    // TODO: We should return a result instead of printing here directly.
    reporting::eprint_errors(
        storage,
        source.id(),
        lexer_errors.iter().chain(parse_errors.iter()),
    );

    schema
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceStorage;

    macro_rules! make_test_from_file {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let mut storage = SourceStorage::new();
                let id = storage.insert(include_str!($path).to_owned(), None);
                let result = parse(&storage, &storage[id]);
                insta::assert_debug_snapshot!(stringify!($name), result);
            }
        };
    }

    make_test_from_file!(
        test_multi_service,
        "../../../examples/simple-multi/bundles/api/schemas/service.sidex"
    );

    make_test_from_file!(
        test_multi_ids,
        "../../../examples/simple-multi/bundles/data/schemas/ids.sidex"
    );

    make_test_from_file!(
        test_multi_person,
        "../../../examples/simple-multi/bundles/data/schemas/person.sidex"
    );

    make_test_from_file!(
        test_single_ids,
        "../../../examples/simple-single/schemas/ids.sidex"
    );

    make_test_from_file!(
        test_single_person,
        "../../../examples/simple-single/schemas/person.sidex"
    );

    make_test_from_file!(
        test_single_task,
        "../../../examples/simple-single/schemas/task.sidex"
    );
}
