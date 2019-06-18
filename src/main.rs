use actix_cors::Cors;
use actix_web::{http::header, http::Method, middleware::Logger, App, HttpServer};

mod todo;
mod todo_endpoints;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let server_address = "127.0.0.1:8000";
    let url = "http://".to_string() + server_address;

    let todo_data = todo_endpoints::TodoData::new(url.to_string());

    HttpServer::new(move || {
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
            .wrap(Logger::default())
            .configure(todo_endpoints::configure)
    })
    .bind(server_address)?
    .run()
}
