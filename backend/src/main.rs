use actix_cors::Cors;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::{middleware, Error};
use listenfd::ListenFd;

mod data;
mod server;

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    server::ws_index(r, stream)
}

async fn echo_json_file(item: web::Json<data::MyJsonFile>) -> HttpResponse {
    server::echo_json_file(item)
}

async fn get_json_file() -> HttpResponse {
    server::get_json_file()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://0.0.0.0:8000")
            .allowed_methods(vec![http::Method::GET])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            ])
            .max_age(3600);

        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // websocket route
            .service(web::resource("/").route(web::get().to(ws_index)))
            // static files
            //.service(fs::Files::new("/", "static/").index_file("index.html"))
            .service(web::resource("/json_post/").route(web::post().to(echo_json_file)))
            .service(web::resource("/json_get/").route(web::get().to(get_json_file)))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = "localhost";
            let port = "8000";
            server.bind(format!("{}:{}", host, port))?
        }
    };
    // start http server on 127.0.0.1:8080
    server.run().await
}
