mod guess;
mod words;

use actix_files::{NamedFile};
use actix_web::{get, post, App, HttpServer, HttpResponse, Result, Responder};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[post("/guess")]
async fn handle_guess(guess_body: String) -> impl Responder {
    match guess::handler(guess_body) {
        Err(why) => HttpResponse::BadRequest().body(why),
        Ok(response) => HttpResponse::Ok().body(response)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(handle_guess)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}