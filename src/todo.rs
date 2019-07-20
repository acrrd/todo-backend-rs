use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub order: Option<u32>,
}

pub type TodoId = Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub completed: bool,
    pub order: u32,
}

impl Todo {
    fn new(id: TodoId, title: String, order: Option<u32>) -> Todo {
        Todo {
            id,
            title,
            completed: false,
            order: order.unwrap_or(0),
        }
    }
}

pub struct TodoStore {
    todos: HashMap<TodoId, Todo>,
}

impl TodoStore {
    pub fn new() -> TodoStore {
        TodoStore {
            todos: HashMap::new(),
        }
    }
}

pub fn get_todos(todo_store: &TodoStore) -> impl Iterator<Item = &Todo> + '_ {
    todo_store.todos.values()
}

pub fn get_todo(todo_store: &TodoStore, id: &TodoId) -> Option<Todo> {
    todo_store.todos.get(id).cloned()
}

pub fn delete_todos(todo_store: &mut TodoStore) {
    todo_store.todos.clear();
}

pub fn delete_todo(todo_store: &mut TodoStore, id: &TodoId) -> Option<Todo> {
    todo_store.todos.remove(id)
}

pub fn create_todo(todo_store: &mut TodoStore, input: CreateTodo) -> Todo {
    let id = Uuid::new_v4();
    let todo = Todo::new(id, input.title, input.order);
    todo_store.todos.insert(id, todo.clone());

    todo
}

pub fn update_todo(todo_store: &mut TodoStore, id: &TodoId, input: UpdateTodo) -> Option<Todo> {
    todo_store.todos.get_mut(&id).map(|todo| {
        input.title.map(|title| {
            todo.title = title;
        });
        input.completed.map(|completed| {
            todo.completed = completed;
        });
        input.order.map(|order| {
            todo.order = order;
        });

        todo.clone()
    })
}
