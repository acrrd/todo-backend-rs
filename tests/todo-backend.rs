use actix_cors::Cors;
use actix_http::HttpService;
use actix_http_test::{TestServer, TestServerRuntime};
use actix_web::{App, http::header, http::Method};

use todo_backend_rs::todo_endpoints;

const URL: &str = "http://testserver:1234/";

fn setup_server() -> TestServerRuntime {
    let todo_data = todo_endpoints::TodoData::new(URL.to_string());

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
                .configure(todo_endpoints::configure),
        )
    })

}

#[test]
fn test_base_endpoint_ok() {
  let mut srv = setup_server();
    let req = srv.get("/todos/");
    let response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
}
