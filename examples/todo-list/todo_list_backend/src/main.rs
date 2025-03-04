use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::post,
};
use sidex_rpc_core::{
    codec::json::Json,
    error::{CallError, InvalidRequestError, InvalidRequestErrorKind},
    request::{self, Request},
    response::Response,
    AsyncCall,
};
use todo_list_data::{
    api::{
        todo_list::{self},
        TodoListService,
    },
    ids::TaskId,
    task::{Status, Task},
};
use uuid::Uuid;

sidex::include_bundle!(pub todo_list_data);

#[derive(Debug, Clone, Default)]
struct TodoListState {
    tasks: HashMap<TaskId, Task>,
}

#[derive(Debug, Clone, Default)]
struct TodoListHandler {
    state: Arc<Mutex<TodoListState>>,
}

impl todo_list_data::api::TodoListHandler for TodoListHandler {
    async fn create(&self, request: Request<todo_list::CreateArgs>) -> Response<TaskId> {
        let task_id = TaskId(Uuid::new_v4());
        self.state.lock().unwrap().tasks.insert(
            task_id.clone(),
            Task::new(request.input.name, Status::Pending),
        );
        Response::from_result(Ok(task_id))
    }

    async fn delete(&self, request: Request<todo_list::DeleteArgs>) -> Response<()> {
        let task = self.state.lock().unwrap().tasks.remove(&request.input.task);
        Response::from_result(if task.is_some() {
            Ok(())
        } else {
            Err(CallError::Invalid(InvalidRequestError::new(
                InvalidRequestErrorKind::Other,
                format_args!("task with id {} does not exist", &request.input.task.0),
            )))
        })
    }

    async fn assign(&self, request: Request<todo_list::AssignArgs>) -> Response<()> {
        todo!()
    }

    async fn unassign(&self, request: Request<todo_list::UnassignArgs>) -> Response<()> {
        todo!()
    }

    async fn set_name(&self, request: Request<todo_list::SetNameArgs>) -> Response<()> {
        todo!()
    }

    async fn set_status(&self, request: Request<todo_list::SetStatusArgs>) -> Response<()> {
        todo!()
    }

    async fn set_description(
        &self,
        request: Request<todo_list::SetDescriptionArgs>,
    ) -> Response<()> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let handler = TodoListHandler::default();
    let service = TodoListService::new(handler);
    let mut router = sidex_rpc::Router::<Json<Vec<u8>>, Json<Vec<u8>>>::new();
    router.register("TodoList", service);

    let app = axum::Router::new().route(
        "/api/v1/rpc/{method}",
        post(
            |Path(method): Path<String>,
             axum::extract::Json(body): axum::extract::Json<serde_json::Value>| {
                async move {
                    println!("method: {method}");
                    let body_vec = Json(serde_json::to_vec(&body).unwrap());
                    let parts = sidex_rpc_core::request::RequestParts::new(method.parse().unwrap());
                    let request = sidex_rpc::core::request::Request::from_parts(parts, body_vec);
                    let response = router.call(request).await;
                    match response.result {
                        Ok(value) => {
                            ([(header::CONTENT_TYPE, "application/json")], value.0).into_response()
                        }
                        Err(error) => {
                            match error {
                                CallError::Invalid(invalid_request_error) => todo!(),
                                CallError::Internal(internal_error) => {
                                    let message = internal_error.message().to_owned();
                                    (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
                                }
                            }
                        }
                    }
                }
            },
        ),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
