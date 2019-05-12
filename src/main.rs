use actix_web::{
    http::header, middleware::cors::Cors, middleware::Logger, web, App, HttpRequest, HttpServer, http::Method
};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
}
