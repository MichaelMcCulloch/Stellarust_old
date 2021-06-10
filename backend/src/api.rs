use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use std::sync::Mutex;

use stellarust::MyJsonFile;

use crate::broadcaster::Broadcaster;

#[get("/events")]
pub async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let client = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(client)
}

#[get("/broadcast/{msg}")]
pub async fn broadcast(msg: Path<String>, broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("Message Sent!")
}

#[post("/json_post")]
pub async fn echo_json_file(item: web::Json<MyJsonFile>) -> impl Responder {
    HttpResponse::Ok().json(item.0)
}

#[get("/json_get")]
pub async fn get_json_file() -> impl Responder {
    let payload = MyJsonFile {
        name: "asdf".into(),
        number: 3,
    };

    HttpResponse::Ok().json(payload)
}
