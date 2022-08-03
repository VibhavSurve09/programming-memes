use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod models;
mod routers;
use dotenv;
extern crate redis;
use redis::Commands;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let con_uri: String = dotenv::var("REDIS").unwrap();

    let client = redis::Client::open(con_uri).unwrap();
    let connection = web::Data::new(Mutex::new(client.get_connection().unwrap()));
    HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .service(routers::memes::get_all_memes)
            .service(routers::memes::get_random_meme)
    })
    .bind((dotenv::var("HOST").unwrap(), 8000))?
    .run()
    .await
}
