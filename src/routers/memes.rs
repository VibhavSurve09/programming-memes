use crate::models;
use actix_web::{get, web, HttpResponse, Responder, Result};
#[get("/")]
pub async fn get_all_memes() -> Result<impl Responder> {
    let memes: Vec<models::memes::Meme> = models::memes::Meme::collect_memes().await;
    return Ok(web::Json(memes));
}

#[get("/random")]
pub async fn get_random_meme() -> impl Responder {
    return HttpResponse::Ok().body("Getting a random meme");
}
