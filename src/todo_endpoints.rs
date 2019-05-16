use actix_web::{web, HttpResponse};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::todo;

#[derive(Clone)]
pub struct TodoData(Arc<RwLock<todo::TodoStore>>);

impl Deref for TodoData {
    type Target = RwLock<todo::TodoStore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TodoData {
    pub fn new() -> Self {
        TodoData(Arc::new(RwLock::new(todo::TodoStore::new())))
    }
}

pub fn get_todos(data: web::Data<TodoData>) -> HttpResponse {
    data.read()
        .map(|store| {
            let todos = todo::get_todos(&store);
            HttpResponse::Ok().json(todos.collect::<Vec<_>>())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub fn delete_todos(data: web::Data<TodoData>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            todo::delete_todos(&mut store);
            HttpResponse::Ok().finish()
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub fn create_todo(data: web::Data<TodoData>, input: web::Json<todo::CreateTodo>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            let todo = todo::create_todo(&mut store, input.into_inner());
            HttpResponse::Ok().json(todo)
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
