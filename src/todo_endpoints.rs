use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::todo;

pub use crate::todo::{CreateTodo, UpdateTodo};

#[derive(Clone)]
pub struct TodoData {
    data: Arc<RwLock<todo::TodoStore>>,
    url: String,
}

impl Deref for TodoData {
    type Target = RwLock<todo::TodoStore>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl TodoData {
    pub fn new(url: String) -> Self {
        TodoData {
            data: Arc::new(RwLock::new(todo::TodoStore::new())),
            url,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: todo::TodoId,
    pub title: String,
    pub completed: bool,
    pub order: u32,
    pub url: String,
}

impl TodoResponse {
    fn new(url: &String, todo: &todo::Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            title: todo.title.clone(),
            completed: todo.completed,
            order: todo.order,
            url: url.clone() + "/todos/" + &todo.id.to_string(),
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todos/")
            .route(web::get().to(get_todos))
            .route(web::delete().to(delete_todos))
            .route(web::post().to(create_todo)),
    );
    cfg.service(
        web::resource("/todos/{id}")
            .route(web::get().to(get_todo))
            .route(web::patch().to(update_todo))
            .route(web::delete().to(delete_todo)),
    );
}

fn get_todos(data: web::Data<TodoData>) -> HttpResponse {
    data.read()
        .map(|store| {
            let todos = todo::get_todos(&store);
            HttpResponse::Ok().json(
                todos
                    .map(|todo| TodoResponse::new(&data.url, todo))
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn get_todo(data: web::Data<TodoData>, id: web::Path<todo::TodoId>) -> HttpResponse {
    data.read()
        .map(|store| {
            let todo = todo::get_todo(&store, &id);
            todo.map(|todo| HttpResponse::Ok().json(TodoResponse::new(&data.url, &todo)))
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

fn delete_todo(data: web::Data<TodoData>, id: web::Path<todo::TodoId>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            let todo = todo::delete_todo(&mut store, &id);
            todo.map(|_| HttpResponse::Ok().finish())
                .unwrap_or(HttpResponse::NotFound().finish())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn create_todo(data: web::Data<TodoData>, input: web::Json<todo::CreateTodo>) -> HttpResponse {
    data.write()
        .map(|mut store| {
            let todo = todo::create_todo(&mut store, input.into_inner());
            HttpResponse::Created().json(TodoResponse::new(&data.url, &todo))
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn update_todo(
    data: web::Data<TodoData>,
    id: web::Path<todo::TodoId>,
    input: web::Json<todo::UpdateTodo>,
) -> HttpResponse {
    data.write()
        .map(|mut store| {
            let todo = todo::update_todo(&mut store, &id, input.into_inner());
            todo.map(|todo| HttpResponse::Ok().json(TodoResponse::new(&data.url, &todo)))
                .unwrap_or(HttpResponse::NotFound().finish())
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
