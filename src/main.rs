use actix_web::{
    http::header, http::Method, middleware::cors::Cors, middleware::Logger, web, App, HttpServer,
};
use std::sync::RwLock;

mod todo;
mod todo_endpoints;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(RwLock::new(todo::TodoStore::new()))
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec![
                        Method::GET,
                        Method::POST,
                        Method::DELETE,
                        Method::OPTIONS,
                    ])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(
                web::resource("/todos/")
                    .route(web::get().to(todo_endpoints::get_todos))
                    .route(web::delete().to(todo_endpoints::delete_todos))
                    .route(web::post().to(todo_endpoints::create_todo)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
}
