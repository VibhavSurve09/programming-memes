use crate::models;
use actix_files::NamedFile;
use actix_web::{get, web, Error, Responder, Result};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
pub async fn get_random_meme() -> Result<NamedFile> {
    let mut image_name: String = String::from("image.");
    let file = File::open("memes.json");
    if let Ok(mut file) = file {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let res: Vec<models::memes::Meme> =
            serde_json::from_str(data.as_str()).expect("Something went");
        let rand_no = generate_random_number();
        let random_meme = res[rand_no as usize].to_owned();
        let response_image_extension = Path::new(&random_meme.link[18..]).extension().unwrap(); //Extracting image extension to send proper headers in response
        image_name.push_str(response_image_extension.to_str().unwrap());
        let mut response_image = File::create(&image_name).unwrap();
        let img = reqwest::get(&random_meme.link)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        response_image.write(&img);
        return Ok(NamedFile::open(image_name)?);
    }
    let res = models::memes::Meme::cache_response()
        .await
        .unwrap()
        .unwrap();
    let rand_no = generate_random_number();
    let random_meme = res[rand_no as usize].to_owned();
    let response_image_extension = Path::new(&random_meme.link[18..]).extension().unwrap(); //Extracting image extension to send proper headers in response
    image_name.push_str(response_image_extension.to_str().unwrap());
    let img = reqwest::get(random_meme.link)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let mut response_image = File::create(&image_name).unwrap();
    response_image.write(&img);
    return Ok(NamedFile::open(image_name)?);
}
