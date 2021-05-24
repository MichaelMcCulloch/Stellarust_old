mod api;
mod broadcaster;
mod directory_watcher;
mod file_reader;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use broadcaster::Broadcaster;
use directory_watcher::DirectoryWatcher;
use file_reader::FileWatcher;
use listenfd::ListenFd;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let dir_watcher = DirectoryWatcher::create("/home/michael/Dev/Stellarust/html_dummy".into());

    let broadcaster_data = Broadcaster::create(dir_watcher.rx);

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec![http::Method::GET])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            ])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(broadcaster_data.clone())
            .wrap(middleware::Logger::default())
            .service(api::echo_json_file)
            .service(api::get_json_file)
            .service(api::new_client)
            .service(api::broadcast)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = "localhost";
            let port = "8000";
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
