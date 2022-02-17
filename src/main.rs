mod words;

use actix_files::{NamedFile};
use actix_web::{get, post, App, HttpServer, HttpResponse, Result, Responder};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[get("/guess")]
async fn guess(guess_body: String) -> impl Responder {
    match words::get_word(0) {
        Err(why) => HttpResponse::BadRequest().body(why),
        Ok(word) => HttpResponse::Ok().body(word)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(guess)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}