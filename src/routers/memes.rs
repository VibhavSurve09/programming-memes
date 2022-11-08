use crate::models;
use actix_files::NamedFile;
use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
extern crate redis;
use redis::Commands;

use rand::seq::SliceRandom;

fn generate_random_number(max_range: i32) -> i32 {
    let secret_number: i32 = rand::thread_rng().gen_range(0..max_range);
    return secret_number;
}
#[get("/")]
pub async fn get_all_memes(
    connection: web::Data<Mutex<redis::Connection>>,
) -> Result<impl Responder> {
    let mut con = connection.lock().unwrap();
    let resp: Result<String, redis::RedisError> = con.get("all_memes");
    match resp {
        Ok(res) => {
            let mut res: Vec<models::memes::Meme> =
                serde_json::from_str(&res).expect("Something went");
            res.shuffle(&mut thread_rng());
            return Ok(web::Json(res));
        }
        _ => {
            let res = models::memes::Meme::cache_response().await.unwrap();
            return Ok(web::Json(res));
        }
    }
}

#[get("/random")]
pub async fn get_random_meme(connection: web::Data<Mutex<redis::Connection>>) -> Result<NamedFile> {
    let mut image_name: String = String::from("image.");
    let mut con = connection.lock().unwrap();
    let resp: Result<String, redis::RedisError> = con.get("all_memes");
    match resp {
        Ok(res) => {
            let res: Vec<models::memes::Meme> =
                serde_json::from_str(res.as_str()).expect("Something went");
            let rand_no = generate_random_number(res.len() as i32);
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
        _ => {
            let res = models::memes::Meme::cache_response().await.unwrap();
            let rand_no = generate_random_number(res.len() as i32);
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
    }
}
