use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        //create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler, todos_list_handler,
        health_checker_handler, pipeline_handler
    },
    //model,
};

pub fn create_router() -> Router {
    //let db = model::todo_db();

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/init_pipe", post(pipeline_handler))
        //.route(
        //    "/api/todos",
        //    post(create_todo_handler).get(todos_list_handler),
        //)
}