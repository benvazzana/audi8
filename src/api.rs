use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransposeParams {
    semitones: i32,
}

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("audi8 api is active")
}

#[post("/transpose")]
async fn transpose_wav(params: web::Query<TransposeParams>, body: web::Bytes) -> impl Responder {
    let size = body.len();
    let num_semitones = params.semitones;
    println!("received {size} bytes");
    println!("transpose request: {num_semitones} semitones");
    HttpResponse::Ok()
}

