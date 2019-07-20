use actix_cors::Cors;
use actix_http::HttpService;
use actix_http_test::{TestServer, TestServerRuntime};
use actix_web::{http::header, http::Method, App};

use todo_backend_rs::todo::{CreateTodo, UpdateTodo};
use todo_backend_rs::todo_endpoints::{configure, TodoData, TodoResponse};

const URL: &str = "http://testserver:1234/";
const TITLE: &str = "title";

fn prepare_url(url: &String) -> String {
    url.chars().skip(URL.len()).collect()
}

fn setup_server() -> TestServerRuntime {
    let todo_data = TodoData::new(URL.to_string());

    TestServer::new(move || {
        HttpService::new(
            App::new()
                .data(todo_data.clone())
                .wrap(
                    Cors::new()
                        .send_wildcard()
                        .allowed_methods(vec![
                            Method::GET,
                            Method::PATCH,
                            Method::POST,
                            Method::DELETE,
                            Method::OPTIONS,
                        ])
                        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                        .allowed_header(header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .configure(configure),
        )
    })
}

macro_rules! get_todos {
    ($srv: ident, $todos: ident, $response: ident) => {
        let req = $srv.get("/todos/").send();

        #[allow(unused_mut)]
        let mut $response = $srv.block_on(req).unwrap();

        assert!(
            $response.status().is_success(),
            "the api root does not responds to a get"
        );

        #[allow(unused_mut)]
        let mut $todos: Vec<TodoResponse> = $srv.block_on($response.json()).unwrap();
    };
    ($srv: ident, $todos: ident) => {
        get_todos!($srv, $todos, _response);
    };
    ($srv: ident) => {
        get_todos!($srv, _todos, _response);
    };
}

macro_rules! create_todo {
    ($srv: ident, $todo: ident, $title: expr, $order: expr) => {
        let req = $srv.post("/todos/").send_json(&CreateTodo {
            title: $title,
            order: $order,
        });

        #[allow(unused_mut)]
        let mut response = $srv.block_on(req).unwrap();

        assert!(
            response.status().is_success(),
            "the api root does not responds to a post"
        );

        #[allow(unused_mut)]
        let mut $todo: TodoResponse = $srv.block_on(response.json()).unwrap();
    };
    ($srv: ident, $todo: ident, $title: expr) => {
        create_todo!($srv, $todo, $title, None);
    };
    ($srv: ident, $todo: ident) => {
        create_todo!($srv, $todo, TITLE.to_string(), None);
    };
    ($srv: ident) => {
        create_todo!($srv, _todo, TITLE.to_string(), None);
    };
}

macro_rules! delete_todos {
    ($srv: ident) => {
        let req = $srv.delete("/todos/").send();

        #[allow(unused_mut)]
        let mut response = $srv.block_on(req).unwrap();

        assert!(
            response.status().is_success(),
            "the api root does not responds to a delete"
        );
    };
}

macro_rules! get_todo {
    ($srv: ident, $url: expr, $todo: ident) => {
        let req = $srv.get(prepare_url($url)).send();

        let mut response = $srv.block_on(req).unwrap();

        assert!(
            response.status().is_success(),
            "the api root does not responds to a get"
        );

        #[allow(unused_mut)]
        let mut $todo: TodoResponse = $srv.block_on(response.json()).unwrap();
    };
}

macro_rules! patch_todo {
    ($srv: ident, $url: expr, $todo: ident, $(@$field: ident : $val: expr),*) => {
        let update_default = UpdateTodo::default();

        let update = UpdateTodo{$($field: Some($val),)* ..update_default};

        let req = $srv.patch(prepare_url($url)).send_json(&update);

        let mut response = $srv.block_on(req).unwrap();

        assert!(
            response.status().is_success(),
            "the api root does not responds to a patch"
        );

        #[allow(unused_mut)]
        let mut $todo: TodoResponse = $srv.block_on(response.json()).unwrap();
    };
    ($srv: ident, $url: expr, $(@$field: ident : $val: expr),*) => {
      patch_todo!($srv, $url, _todo, $(@$field : $val),*);
    };
}

macro_rules! delete_todo {
    ($srv: ident, $url: expr) => {
        let req = $srv.delete(prepare_url($url)).send();

        #[allow(unused_mut)]
        let mut response = $srv.block_on(req).unwrap();

        assert!(
            response.status().is_success(),
            "the api root does not responds to a direct delete"
        );
    };
}

macro_rules! title_check {
    ($todo: ident) => {
        assert_eq!(
            TITLE.to_string(),
            $todo.title,
            "the api root does not return the correct title"
        );
    };
    ($todo: ident, $($title: expr),*) => {
        assert!(
          $($todo.title == $title ||)* false,
            "the api root does not return the correct title"
        );
    };
}

#[test]
fn responds_to_a_get() {
    let mut srv = setup_server();

    get_todos!(srv);
}

#[test]
fn responds_to_a_post_with_the_todo_which_was_posted_to_it() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    title_check!(todo);
}

#[test]
fn responds_successfully_to_a_delete() {
    let mut srv = setup_server();

    delete_todos!(srv);
}

#[test]
fn after_a_delete_responds_to_a_get_with_an_empty_array() {
    let mut srv = setup_server();

    create_todo!(srv);

    delete_todos!(srv);

    get_todos!(srv, todos);

    assert!(
        todos.is_empty(),
        "the api root does not return and empty array after delete"
    );
}

#[test]
fn add_a_new_todo_to_the_list_of_todos() {
    let mut srv = setup_server();

    create_todo!(srv);
    get_todos!(srv, todos);

    assert_eq!(todos.len(), 1, "the list of todos should contains 1 todo");

    let todo = todos.get(0).unwrap();
    title_check!(todo);
}

#[test]
fn add_a_new_todo_as_not_completed() {
    let mut srv = setup_server();

    create_todo!(srv);
    get_todos!(srv, todos);

    let todo = todos.get(0).unwrap();
    assert_eq!(todo.completed, false, "inserted todo is already completed");
}

#[test]
fn each_new_todo_has_a_url() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    assert!(todo.url.len() > 0, "inserted todo does not have an url");
}

#[test]
fn each_new_todo_has_a_url_which_returns_a_todo() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    get_todo!(srv, &todo.url, todo);
    title_check!(todo);
}

#[test]
fn can_navigate_from_todos_to_individual_todo_via_url() {
    let mut srv = setup_server();

    create_todo!(srv);
    create_todo!(srv, _todo, "second title".into());
    get_todos!(srv, todos);

    assert_eq!(todos.len(), 2, "the list of todos should contains 2 todos");
    let todo = todos.get(0).unwrap();

    get_todo!(srv, &todo.url, todo);
    title_check!(todo, TITLE, "second title");
}

#[test]
fn can_change_the_title_by_patch_to_the_url() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    let url = &todo.url;
    let title = "new title".to_string();
    patch_todo!(srv, url, todo, @title: title.clone());

    title_check!(todo, title);
}

#[test]
fn can_change_the_completeness_by_patch_to_the_url() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    let url = &todo.url;
    patch_todo!(srv, url, todo, @completed: true);

    assert!(todo.completed, "completed has not been modified");
}

#[test]
fn changes_to_a_todo_are_persisted_and_show_up_on_refetching() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    let title = "new title".to_string();
    patch_todo!(srv, &todo.url, @title: title.clone(), @completed: true);

    get_todo!(srv, &todo.url, todo);
    title_check!(todo, title);
    assert!(todo.completed, "completed has not been modified");

    get_todos!(srv, todos);

    let todo = todos.get(0).unwrap();
    title_check!(todo, title);
    assert!(todo.completed, "completed has not been modified");
}

#[test]
fn can_delete_a_todo_with_a_request_to_url() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    delete_todo!(srv, &todo.url);

    get_todos!(srv, todos);
    assert_eq!(todos.len(), 0, "todo has not been deleted");
}

#[test]
fn create_a_todo_with_an_order_field() {
    let mut srv = setup_server();

    let order = 1;
    create_todo!(srv, todo, TITLE.to_string(), Some(order));
    assert_eq!(todo.order, order);

    get_todo!(srv, &todo.url, todo);
    assert_eq!(todo.order, order);
}

#[test]
fn can_patch_a_todo_to_change_its_order() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    let order = 1;
    patch_todo!(srv, &todo.url, todo, @order: order);

    assert_eq!(todo.order, order);
}

#[test]
fn remembers_changes_to_a_todo_order() {
    let mut srv = setup_server();

    create_todo!(srv, todo);

    let order = 1;
    patch_todo!(srv, &todo.url, @order: order);

    get_todo!(srv, &todo.url, todo);
    assert_eq!(todo.order, order);
}
