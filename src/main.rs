use actix_web::{
    http::header, http::Method, middleware::cors::Cors, middleware::Logger, web, App, HttpRequest,
    HttpResponse, HttpServer,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    title: String,
}

fn get_todos() -> HttpResponse {
    HttpResponse::Ok().json([] as [Todo; 0])
}

fn delete_todos() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn create_todo(todo: web::Json<Todo>) -> HttpResponse {
    HttpResponse::Ok().json(todo.0)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
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
                    .route(web::get().to(get_todos))
                    .route(web::delete().to(delete_todos))
                    .route(web::post().to(create_todo)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
}
