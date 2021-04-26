use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use broadcaster::Broadcaster;
use listenfd::ListenFd;

mod broadcaster;
mod responder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let broadcaster_data = Broadcaster::create();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(broadcaster_data.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/json_post").route(web::post().to(responder::echo_json_file)))
            .service(web::resource("/json_get").route(web::get().to(responder::get_json_file)))
            .service(web::resource("/events").route(web::get().to(responder::new_client)))
            .service(web::resource("/broadcast/{msg}").route(web::get().to(responder::broadcast)))
            .service(Files::new("/", "./static/").index_file("index.html"))
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
