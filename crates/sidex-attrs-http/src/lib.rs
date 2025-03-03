use sidex_gen::{
    attrs::{AttrConvertExt, TryApplyAttr, TryFromAttr, TryFromAttrs, new_assign_attr, reject},
    diagnostics::{self, Diagnostic},
    ir,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

pub struct HttpOperationAttrs {
    pub method: HttpMethod,
    pub path: Option<String>,
}

impl TryFromAttrs for HttpOperationAttrs {
    fn try_from_attrs<'a, I: IntoIterator<Item = &'a ir::Attr>>(
        attrs: I,
    ) -> diagnostics::Result<Self> {
        for attr in attrs {
            let Ok(list) = attr.expect_list_with("http") else {
                continue;
            };
            if list.args.len() != 1 {
                reject!(attr, "invalid number of arguments for http attribute");
            }
            let (method, path) = if let Ok(inner) = list.args[0].expect_list() {
                (inner.path.as_str(), inner.args.get(0))
            } else {
                (list.args[0].expect_path()?.as_str(), None)
            };
            let method = match method {
                "get" => HttpMethod::Get,
                "put" => HttpMethod::Put,
                "post" => HttpMethod::Post,
                "delete" => HttpMethod::Delete,
                "options" => HttpMethod::Options,
                "head" => HttpMethod::Head,
                "patch" => HttpMethod::Patch,
                "trace" => HttpMethod::Trace,
                _ => {
                    reject!(attr, "invalid HTTP method");
                }
            };
            let path = path
                .map(|attr| attr.expect_string_literal())
                .transpose()?
                .map(str::to_owned);

            return Ok(HttpOperationAttrs { method, path });
        }
        Err(Diagnostic::error("did not find any http attribute"))
    }
}

#[derive(Clone, Default)]
pub struct HttpInterfaceAttrs {
    pub path: String,
    pub open_api: OpenApiInterfaceAttrs,
}

impl TryFromAttrs for HttpInterfaceAttrs {
    fn try_from_attrs<'a, I: IntoIterator<Item = &'a ir::Attr>>(
        attrs: I,
    ) -> diagnostics::Result<Self> {
        let mut open_api = OpenApiInterfaceAttrs::default();
        let mut path = None;
        for attr in attrs {
            let Ok(list) = attr.expect_list_with("http") else {
                open_api.try_apply_attr(attr).ok();
                continue;
            };
            if list.args.len() != 1 {
                reject!(attr, "invalid number of arguments for http attribute");
            }
            path = Some(list.args[0].expect_string_literal()?.to_owned());
        }
        if let Some(path) = path {
            Ok(HttpInterfaceAttrs { path, open_api })
        } else {
            Err(Diagnostic::error("did not find any http attribute"))
        }
    }
}

#[derive(Clone, Default)]
pub struct OpenApiInterfaceAttrs {
    pub tag: Option<String>,
}

new_assign_attr! {
    /// An attribute of the form `tag = "<TAG>"`.
    pub struct TagAttr["tag"](pub String)
}

impl TryApplyAttr for OpenApiInterfaceAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        let list = attr.expect_list_with("openapi")?;
        for attr in &list.args {
            if let Ok(tag) = TagAttr::try_from_attr(attr) {
                self.tag = Some(tag.0);
            }
        }
        Ok(())
    }
}
