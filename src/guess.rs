extern crate serde;

use serde_json::{Value};
use serde::{Serialize, Deserialize};
use std::iter::Iterator;
use crate::words::{get_word, get_random_word};

const LETTER_STATUS_CORRECT: &str = "CORRECT";
const LETTER_STATUS_ALMOST: &str = "ALMOST";
const LETTER_STATUS_WRONG: &str = "WRONG";

#[derive(Serialize, Deserialize, Debug)]
struct GuessRequest {
    guess: String,
    word: u32   ,
}

#[derive(Serialize, Deserialize, Debug)]
struct GuessResponse {
    result: Vec<String>,
    word: u32,
    win: bool,
}

fn check_guess(request: GuessRequest) -> Result<GuessResponse, String> {
    match get_word(request.word) {
        Ok(word) => {
            if word.len() != request.guess.len() {
                return Err(format!("Guess should have {} letters (was {})", word.len(), request.guess));
            }
            let mut word_chars: Vec<char> = word.chars().collect();
            let guess_chars: Vec<char> = request.guess.chars().collect();
            let mut result = vec![];

            'letters: for index in 0..word_chars.len() {
                let guessed = guess_chars[index];
                let actual = word_chars[index];

                if guessed == actual {
                    result.push(LETTER_STATUS_CORRECT.to_string());
                    word_chars[index] = '_';
                } else {
                    // Check if guessed letter has a match in another position
                    // in the target word, but ONLY if that letter isn't correctly
                    // matched, and ONLY if the out-of-position match hasn't
                    // already been matched to another guessed letter
                    for actual_char_index in 0..word_chars.len() {
                        if word_chars[actual_char_index] != guess_chars[actual_char_index] && word_chars[actual_char_index] == guessed {
                            result.push(LETTER_STATUS_ALMOST.to_string());
                            word_chars[actual_char_index] = '_';
                            continue 'letters;
                        }
                    }
                    result.push(LETTER_STATUS_WRONG.to_string());
                }
            }

            let win = result.iter().all(|r| r == &LETTER_STATUS_CORRECT.to_string()); 

            Ok(GuessResponse {
                result,
                word: request.word,
                win,
            })
        },
        Err(why) => Err(why)
    }
}

pub fn handler(request: String) -> Result<String, String> {
    let parsed_request: Value = serde_json::from_str(&request).map_err(|_| "Error parsing JSON".to_string())?;

    if let Value::Null = parsed_request["guess"] {
        return Err("Missing guess parameter".to_string());
    }

    let typed_request: GuessRequest = if let Value::Null = parsed_request["word"] {
        GuessRequest {
            guess: parsed_request["guess"].to_string(),
            word: get_random_word()?,
        }
    } else {
        serde_json::from_str(&request).map_err(|_| "Error parsing JSON".to_string())?
    };

    serde_json::to_string(&check_guess(typed_request)?).map_err(|_| "Error rendering JSON".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win() -> Result<(), String> {
        let response = check_guess(GuessRequest {
            guess: "which".to_string(),
            word: 0,
        })?;
        
        assert_eq!(response.result, vec!["CORRECT", "CORRECT", "CORRECT", "CORRECT", "CORRECT",]);
        assert_eq!(response.win, true);
        Ok(())
    }

    #[test]
    fn test_loss() -> Result<(), String> {
        let response = check_guess(GuessRequest {
            guess: "qualm".to_string(),
            word: 0,
        })?;
        assert_eq!(response.result, vec!["WRONG", "WRONG", "WRONG", "WRONG", "WRONG",]);
        assert_eq!(response.win, false);
        Ok(())
    }

    #[test]
    fn test_partial_loss() -> Result<(), String> {
        let response = check_guess(GuessRequest {
            guess: "whole".to_string(),
            word: 0,
        })?;
        assert_eq!(response.result, vec!["CORRECT", "CORRECT", "WRONG", "WRONG", "WRONG",]);
        assert_eq!(response.win, false);
        Ok(())
    }

    #[test]
    fn test_almost_characters() -> Result<(), String> {
        let response = check_guess(GuessRequest {
            guess: "whhhe".to_string(),
            word: 0,
        })?;
        assert_eq!(response.result, vec!["CORRECT", "CORRECT", "ALMOST", "WRONG", "WRONG",]);
        assert_eq!(response.win, false);
        Ok(())
    }

    #[test]
    fn test_handler() -> Result<(), String> {
        let response = handler("{ \"guess\": \"which\", \"word\": 0 }".to_string())?;
        assert_eq!(response, "{\"result\":[\"CORRECT\",\"CORRECT\",\"CORRECT\",\"CORRECT\",\"CORRECT\"],\"word\":0,\"win\":true}");
        Ok(())
    }
}