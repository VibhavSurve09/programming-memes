use actix_web::{App, HttpServer};

mod models;
mod routers;
use dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routers::memes::get_all_memes)
            .service(routers::memes::get_random_meme)
    })
    .bind((dotenv::var("HOST").unwrap(), 8000))?
    .run()
    .await
}
