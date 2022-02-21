mod words;
mod guess;

#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::response::{status};
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GuessRequest<'r> {
    guess: &'r str,
    word: Option<usize>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GuessResponse<'a> {
    result: Vec<&'a str>,
    word: usize,
    win: bool,
}

#[post("/guess", format = "json", data = "<message>")]
async fn handle_guess(message: Json<GuessRequest<'_>>) -> Result<Json<GuessResponse<'_>>, status::BadRequest<String>> {
    let word = message.word.unwrap_or(words::get_random_word().map_err(|err| status::BadRequest(Some(err)))?);
    let (result, win) = guess::check_guess(message.guess, word).map_err(|err| status::BadRequest(Some(err)))?;
    Ok(Json(GuessResponse {
        result,
        word,
        win
    }))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static/")))
        .mount("/", routes![handle_guess])
}