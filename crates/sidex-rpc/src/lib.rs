use std::{collections::HashMap, pin::Pin, sync::Arc};

pub use sidex_rpc_core as core;
use sidex_rpc_core::{
    AsyncCall,
    error::{CallError, InvalidRequestError, InvalidRequestErrorKind},
    request::Request,
    response::Response,
};

pub struct Router<I, O> {
    services: HashMap<
        String,
        Arc<dyn Send + Sync + Fn(Request<I>) -> Pin<Box<dyn Send + Future<Output = Response<O>>>>>,
    >,
}

impl<I, O> Clone for Router<I, O> {
    fn clone(&self) -> Self {
        Self {
            services: self.services.clone(),
        }
    }
}

impl<I, O> Router<I, O> {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<S: AsyncCall<I, O>>(&mut self, prefix: &str, service: S) {
        self.services.insert(
            prefix.to_owned(),
            Arc::new(move |request| Box::pin(service.call(request))),
        );
    }
}

impl<I: 'static, O: 'static> AsyncCall<I, O> for Router<I, O> {
    type Future = Pin<Box<dyn Future<Output = Response<O>> + Send>>;

    fn call(&self, mut request: Request<I>) -> Self::Future {
        let (prefix, method) = request.parts.method.split_last();
        if let Some(service) = self.services.get(prefix) {
            println!("service found, method: {}", method.as_str());
            request.parts.method = method;
            service(request)
        } else {
            Box::pin(async move {
                Response::from_result(Err(CallError::Invalid(InvalidRequestError::new(
                    InvalidRequestErrorKind::NotFound,
                    format_args!("method {} not found", request.parts.method.as_str()),
                ))))
            })
        }
    }
}
