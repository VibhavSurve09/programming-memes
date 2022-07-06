use crate::models;
use actix_web::{get, http::header::ContentType, web, Error, HttpResponse, Responder, Result};
use futures::{future::ok, stream::once};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
fn generate_random_number() -> i32 {
    let secret_number: i32 = rand::thread_rng().gen_range(0..405);
    return secret_number;
}
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
pub async fn get_random_meme() -> HttpResponse {
    let mut file = File::open("memes.json");
    if let Ok(mut file) = file {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let res: Vec<models::memes::Meme> =
            serde_json::from_str(data.as_str()).expect("Something went");
        let rand_no = generate_random_number();
        let random_meme = res[rand_no as usize].to_owned();
        let img = reqwest::get(random_meme.link)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let body = once(ok::<_, Error>(img));
        return HttpResponse::Ok().streaming(body);
    }
    let res = models::memes::Meme::cache_response()
        .await
        .unwrap()
        .unwrap();
    let rand_no = generate_random_number();
    let random_meme = res[rand_no as usize].to_owned();
    let img = reqwest::get(random_meme.link)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let body = once(ok::<_, Error>(img));
    return HttpResponse::Ok().streaming(body);
}
