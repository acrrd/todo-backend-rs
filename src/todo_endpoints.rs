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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todos/")
            .route(web::get().to(get_todos))
            .route(web::delete().to(delete_todos))
            .route(web::post().to(create_todo)),
    );
    cfg.service(web::resource("/todos/{id}").route(web::get().to(get_todo)));
}

fn get_todos(data: web::Data<TodoData>) -> HttpResponse {
    data.read()
        .map(|store| {
            let todos = todo::get_todos(&store);
            HttpResponse::Ok().json(todos.collect::<Vec<_>>())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn get_todo(data: web::Data<TodoData>, id: web::Path<todo::TodoId>) -> HttpResponse {
    data.read()
        .map(|store| {
            let todo = todo::get_todo(&store, &id);
            todo.map(|todo| HttpResponse::Ok().json(todo))
                .unwrap_or(HttpResponse::NotFound().finish())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn delete_todos(data: web::Data<TodoData>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            todo::delete_todos(&mut store);
            HttpResponse::Ok().finish()
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn make_todo_url<TodoId: ToString>(id: &TodoId) -> String {
    let prefix: String = "http://127.0.0.1:8000/todos/".to_string();
    prefix + &id.to_string()
}

fn create_todo(data: web::Data<TodoData>, input: web::Json<todo::CreateTodo>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            let todo = todo::create_todo(&mut store, input.into_inner(), make_todo_url);
            HttpResponse::Created().json(todo)
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
