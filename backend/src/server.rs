use actix_web::{
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use std::sync::Mutex;

extern crate common;
use common::MyJsonFile;

use super::broadcaster::Broadcaster;

pub async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}

pub async fn broadcast(msg: Path<String>, broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

pub async fn echo_json_file(item: web::Json<MyJsonFile>) -> impl Responder {
    HttpResponse::Ok().json(item.0)
}

pub async fn get_json_file() -> impl Responder {
    let payload = MyJsonFile {
        name: "asdf".into(),
        number: 3,
    };

    HttpResponse::Ok().json(payload)
}
