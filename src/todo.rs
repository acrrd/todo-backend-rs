use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateTodo {
    title: String,
}

type TodoId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    id: TodoId,
    title: String,
    completed: bool,
    url: String,
}

impl Todo {
    fn new(id: TodoId, title: String, url: String) -> Todo {
        Todo {
            id,
            title,
            completed: false,
            url,
        }
    }
}

pub struct TodoStore {
    next_id: TodoId,
    todos: HashMap<TodoId, Todo>,
}

impl TodoStore {
    pub fn new() -> TodoStore {
        TodoStore {
            next_id: 0,
            todos: HashMap::new(),
        }
    }
}

pub fn get_todos(todo_store: &TodoStore) -> impl Iterator<Item = &Todo> + '_ {
    todo_store.todos.values()
}

pub fn delete_todos(todo_store: &mut TodoStore) {
    todo_store.todos.clear();
}

pub fn create_todo(
    todo_store: &mut TodoStore,
    input: CreateTodo,
    get_url: impl (FnOnce(&TodoId) -> String),
) -> Todo {
    let id = todo_store.next_id;
    todo_store.next_id += 1;
    let url = get_url(&id);
    let todo = Todo::new(id, input.title, url);
    todo_store.todos.insert(id, todo.clone());

    todo
}
