use crate::models;

use actix_web::{get, web, HttpResponse, Responder, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[get("/")]
pub async fn get_all_memes() -> Result<impl Responder> {
    let mut file = File::open("memes.json");
    if let Ok(mut file) = file {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let res = serde_json::from_str(data.as_str()).expect("Something went");
        return Ok(web::Json(res));
    }
    let res = models::memes::Meme::cache_response()
        .await
        .unwrap()
        .unwrap();
    return Ok(web::Json(res));
}

#[get("/random")]
pub async fn get_random_meme() -> impl Responder {
    return HttpResponse::Ok().body("Getting a random meme");
}
