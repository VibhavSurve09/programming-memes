use crate::models;
use actix_web::{get, HttpResponse, Responder};
#[get("/")]
pub async fn get_all_memes() -> impl Responder {
    let memes: Vec<models::memes::Meme> = models::memes::Meme::collect_memes().await;
    println!("Awating {:?}", memes);
    return HttpResponse::Ok().body("Getting all memes");
}

#[get("/random")]
pub async fn get_random_meme() -> impl Responder {
    return HttpResponse::Ok().body("Getting a random meme");
}
