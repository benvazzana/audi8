use std::io::Cursor;

use actix_web::{get, http::header::CONTENT_TYPE, post, web, HttpResponse, Responder};
use hound::{WavReader, WavSpec, WavWriter};
use serde::Deserialize;

use crate::transpose;

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

    let mut reader = WavReader::new(Cursor::new(body)).unwrap();

    let spec = reader.spec();
    let mut out_buf = Vec::<u8>::new();
    let mut writer = WavWriter::new(Cursor::new(&mut out_buf),
                                    WavSpec {
                                        channels: spec.channels,
                                        sample_rate: spec.sample_rate,
                                        bits_per_sample: 16,
                                        sample_format: hound::SampleFormat::Int,
                                    }).unwrap();
    transpose::transpose_audio(&mut reader, &mut writer, num_semitones);
    writer.finalize().unwrap();

    HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, "audio/wav"))
        .body(out_buf)

}

