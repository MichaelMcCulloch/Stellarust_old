mod api;
mod broadcaster;
mod file_reader;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use broadcaster::Broadcaster;

#[cfg(target_os = "linux")]
mod linux_directory_watcher;
#[cfg(target_os = "linux")]
use linux_directory_watcher::DirectoryWatcher;

#[cfg(target_os = "windows")]
use windows_directory_watcher::DirectoryWatcher;
#[cfg(target_os = "windows")]
mod windows_directory_watcher;

#[cfg(target_os = "macos")]
use macos_directory_watcher::DirectoryWatcher;
#[cfg(target_os = "macos")]
mod macos_directory_watcher;

use file_reader::FileReader;
use listenfd::ListenFd;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let dir_path = "/home/michael/Dev/Stellarust/html_dummy";

    let directory_watcher = DirectoryWatcher::create(dir_path.into());
    let file_reader = FileReader::create(directory_watcher.pathbuf_receiver);
    let broadcaster_data = Broadcaster::create(file_reader.file_content_receiver);

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
