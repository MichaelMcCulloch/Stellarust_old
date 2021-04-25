use actix_web::{
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use std::sync::Mutex;

extern crate common;
use common::MyJsonFile;

use super::broadcaster::Broadcaster;

async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}

async fn broadcast(msg: Path<String>, broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

pub fn echo_json_file(item: web::Json<MyJsonFile>) -> HttpResponse {
    HttpResponse::Ok().json(item.0)
}

pub fn get_json_file() -> HttpResponse {
    let payload = MyJsonFile {
        name: "asdf".into(),
        number: 3,
    };

    HttpResponse::Ok().json(payload)
}

pub fn config_server(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/json_post").route(web::post().to(echo_json_file)));
    cfg.service(web::resource("/json_get").route(web::get().to(get_json_file)));
    cfg.service(web::resource("/events").route(web::get().to(new_client)));
    cfg.service(web::resource("/broadcast/{msg}").route(web::get().to(broadcast)));
}
