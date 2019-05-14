use actix_web::{web, HttpResponse};
use std::sync::RwLock;

use crate::todo;

pub fn get_todos(store_lock: web::Data<RwLock<todo::TodoStore>>) -> HttpResponse {
    store_lock
        .read()
        .map(|store| {
            let todos = todo::get_todos(&store);
            HttpResponse::Ok().json(todos.collect::<Vec<_>>())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub fn delete_todos(store_lock: web::Data<RwLock<todo::TodoStore>>) -> HttpResponse {
    store_lock
        .write()
        .map(|mut store| {
            todo::delete_todos(&mut store);
            HttpResponse::Ok().finish()
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub fn create_todo(
    store_lock: web::Data<RwLock<todo::TodoStore>>,
    input: web::Json<todo::CreateTodo>,
) -> HttpResponse {
    store_lock
        .write()
        .map(|mut store| {
            let todo = todo::create_todo(&mut store, input.into_inner());
            HttpResponse::Ok().json(todo)
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
