use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use sidex_attrs_http::{HttpInterfaceAttrs, HttpMethod, HttpOperationAttrs};
use sidex_gen::{attrs::TryFromAttrs, diagnostics, ir, Generator};

#[doc(hidden)]
mod generated;
pub use generated::openapi as types;

use crate::types::PathItem;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub title: Option<String>,
    pub info: Option<types::Info>,
    pub tags: Option<Vec<types::Tag>>,
    pub servers: Option<Vec<types::Server>>,
}

#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: String,
    pub tree: PathTree,
}

#[derive(Debug, Clone)]
pub struct Operation {}

#[derive(Debug, Clone, Default)]
pub struct PathTree {
    pub operations: HashMap<HttpMethod, types::Operation>,
    pub children: IndexMap<String, PathTree>,
    pub catch: Option<Box<PathParam>>,
}

impl PathTree {
    pub fn resolve_mut(&mut self, path: &str) -> &mut PathTree {
        let mut resolved = self;
        for part in path.split("/") {
            if part.starts_with("{") {
                if !part.ends_with("}") {
                    panic!("invalid part");
                }
                let param_name = part.trim_matches(|c| c == '{' || c == '}');
                resolved = &mut resolved
                    .catch
                    .get_or_insert_with(|| {
                        Box::new(PathParam {
                            name: param_name.to_owned(),
                            tree: PathTree::default(),
                        })
                    })
                    .tree
            } else {
                resolved = resolved.children.entry(part.to_owned()).or_default();
            }
        }
        resolved
    }
}

pub struct OpenApiGenerator;

impl Generator for OpenApiGenerator {
    fn generate(&self, job: sidex_gen::Job) -> diagnostics::Result<()> {
        //todo!()
        let openapi_file = job.output.join("openapi.yaml");

        let metadata = &job.unit[job.bundle].metadata;
        let config = Config::deserialize(job.config.clone().into_deserializer())?;

        let mut openapi = types::OpenApi::new(
            "3.1.0".to_owned(),
            config.info.unwrap_or_else(|| {
                types::Info::new(
                    config.title.as_deref().unwrap_or("Untitled").to_owned(),
                    metadata.version.clone(),
                )
                .with_description(
                    metadata
                        .description
                        .as_deref()
                        .map(|description| types::Markdown(description.to_owned())),
                )
            }),
        );

        let mut root = PathTree::default();

        let bundle = &job.unit[job.bundle];
        for schema in &bundle.schemas {
            for def in &schema.defs {
                let ir::DefKind::Interface(interface) = &def.kind else {
                    continue;
                };
                let Ok(interface_attrs) = HttpInterfaceAttrs::try_from_attrs(&def.attrs) else {
                    continue;
                };
                let path = &interface_attrs.path;
                println!(
                    "Found HTTP interface `{}` for `{}`.",
                    def.name.identifier, path
                );

                for method in &interface.methods {
                    let operation_attrs = match HttpOperationAttrs::try_from_attrs(&method.attrs) {
                        Ok(attrs) => attrs,
                        Err(error) => {
                            println!("{:?}", error);
                            continue;
                        }
                    };
                    println!("Found HTTP operation `{}`.", method.name.identifier);
                    let mut tree = root.resolve_mut(path.strip_prefix("/").unwrap());
                    if let Some(path) = &operation_attrs.path {
                        tree = tree.resolve_mut(path.strip_prefix("/").unwrap());
                    };
                    let mut operation = types::Operation::new();
                    if let Some(tag) = &interface_attrs.open_api.tag {
                        operation.set_tags(Some(vec![tag.clone()]));
                    }
                    if let Some(docs) = &method.docs {
                        operation.set_summary(Some(docs.as_str().to_owned()));
                    }
                    let mut responses = IndexMap::new();
                    responses.insert(
                        "200".to_owned(),
                        types::MaybeRef::Value(types::Response::new(types::Markdown(
                            "XYZ".to_owned(),
                        ))),
                    );
                    operation.set_responses(Some(types::Responses(responses)));
                    tree.operations.insert(operation_attrs.method, operation);
                }
            }
        }

        let mut paths = IndexMap::new();

        let mut stack = vec![("".to_owned(), vec![], &root)];
        while let Some((path, params, tree)) = stack.pop() {
            let mut item = types::PathItem::new();
            for (method, operation) in &tree.operations {
                let mut operation = operation.clone();
                operation.set_parameters(Some(params.clone()));
                match method {
                    HttpMethod::Get => item.set_get(Some(operation)),
                    HttpMethod::Put => item.set_put(Some(operation)),
                    HttpMethod::Post => item.set_post(Some(operation)),
                    HttpMethod::Delete => item.set_delete(Some(operation)),
                    HttpMethod::Options => item.set_options(Some(operation)),
                    HttpMethod::Head => item.set_head(Some(operation)),
                    HttpMethod::Patch => item.set_patch(Some(operation)),
                    HttpMethod::Trace => item.set_trace(Some(operation)),
                };
            }
            if !tree.operations.is_empty() {
                paths.insert(
                    if path.is_empty() {
                        "/".to_owned()
                    } else {
                        path.clone()
                    },
                    item,
                );
            }
            if let Some(catch) = &tree.catch {
                let name = sidex_gen::rename::to_camel_case(&catch.name);
                let mut params = params.clone();
                params.push(types::MaybeRef::Value(
                    types::Parameter::new(name.clone(), types::ParameterLocation::Path)
                        .with_required(Some(true))
                        .with_schema(Some(types::Schema(types::Any::Object(
                            [
                                ("type".to_owned(), types::Any::String("integer".to_owned())),
                                ("format".to_owned(), types::Any::String("int64".to_owned())),
                            ]
                            .into_iter()
                            .collect(),
                        )))),
                ));
                stack.push((format!("{path}/{{{}}}", name), params, &catch.tree))
            }
            for (fragment, child_tree) in &tree.children {
                stack.push((format!("{path}/{fragment}"), params.clone(), child_tree));
            }
        }

        openapi.set_paths(Some(types::Paths(paths)));
        openapi.set_tags(config.tags.clone());
        openapi.set_servers(config.servers.clone());

        println!("{:#?}", root);

        std::fs::write(openapi_file, serde_yaml::to_string(&openapi)?)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    macro_rules! test_json_input {
        ($name:ident, $type:ty, $file:literal) => {
            #[test]
            pub fn $name() {
                serde_json::from_str::<$type>(include_str!($file)).unwrap();
            }
        };
    }

    test_json_input!(
        petstore_schema,
        types::OpenApi,
        "../tests/petstore/openapi.json"
    );
    test_json_input!(
        petstore_part_parameter,
        types::Parameter,
        "../tests/petstore/parts/parameter.json"
    );
    test_json_input!(
        petstore_part_response,
        types::Response,
        "../tests/petstore/parts/response.json"
    );
}
