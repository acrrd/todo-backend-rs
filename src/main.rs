use actix_web::{
    http::header, http::Method, middleware::cors::Cors, middleware::Logger, App, HttpServer,
};

mod todo;
mod todo_endpoints;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let todo_data = todo_endpoints::TodoData::new();

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
    .bind("127.0.0.1:8000")?
    .run()
}
