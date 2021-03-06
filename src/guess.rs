use crate::words::{get_word, is_valid_word};

pub fn check_guess<'a,'b>(guess: &'a str, word_index: usize) -> Result<(Vec<&'b str>, bool), String> {
    if !is_valid_word(guess) {
        return Err("Guess is not a valid word".to_string());
    }
    match get_word(word_index) {
        Some(word) => {
            let comparison = compare_words(word, guess)?;
            let win = comparison.iter().all(|r| *r == "CORRECT");

            Ok((comparison, win))
        },
        None => Err("Word not found".to_string())
    }
}
fn compare_words<'a,'b>(actual: &'a str, guess: &'a str) -> Result<Vec<&'b str>, String> {
    if actual.len() != guess.len() {
        return Err(format!("Guess should have {} letters (was {})", actual.len(), guess));
    }
    let mut word_chars: Vec<char> = actual.to_lowercase().chars().collect();
    let guess_chars: Vec<char> = guess.to_lowercase().chars().collect();
    let mut result = vec![];

    'letters: for index in 0..word_chars.len() {
        let guessed = guess_chars[index];
        let actual = word_chars[index];

        if guessed == actual {
            result.push("CORRECT");
            word_chars[index] = '_';
        } else {
            // Check if guessed letter has a match in another position
            // in the target word, but ONLY if that letter isn't correctly
            // matched, and ONLY if the out-of-position match hasn't
            // already been matched to another guessed letter
            for actual_char_index in 0..word_chars.len() {
                if word_chars[actual_char_index] != guess_chars[actual_char_index] && word_chars[actual_char_index] == guessed {
                    result.push("ALMOST");
                    word_chars[actual_char_index] = '_';
                    continue 'letters;
                }
            }
            result.push("WRONG");
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win() -> Result<(), String> {
        let response = compare_words("which", "which")?;
        
        assert_eq!(response, vec!["CORRECT", "CORRECT", "CORRECT", "CORRECT", "CORRECT",]);
        Ok(())
    }

    #[test]
    fn test_case() -> Result<(), String> {
        let response = compare_words("Which", "WHICH")?;
        
        assert_eq!(response, vec!["CORRECT", "CORRECT", "CORRECT", "CORRECT", "CORRECT",]);
        Ok(())
    }

    #[test]
    fn test_loss() -> Result<(), String> {
        let response = compare_words("which", "qualm")?;
        assert_eq!(response, vec!["WRONG", "WRONG", "WRONG", "WRONG", "WRONG",]);
        Ok(())
    }

    #[test]
    fn test_partial_loss() -> Result<(), String> {
        let response = compare_words("which", "whole")?;
        assert_eq!(response, vec!["CORRECT", "CORRECT", "WRONG", "WRONG", "WRONG",]);
        Ok(())
    }

    #[test]
    fn test_almost_characters() -> Result<(), String> {
        let response = compare_words("which", "whhhe")?;
        assert_eq!(response, vec!["CORRECT", "CORRECT", "ALMOST", "WRONG", "WRONG",]);
        Ok(())
    }

    #[test]
    fn test_invalid_word() -> Result<(), String> {
        let response = check_guess("whhhe", 0);
        assert_eq!(response, Err("Guess is not a valid word".to_string()));
        Ok(())
    }

    #[test]
    fn test_invalid_index() -> Result<(), String> {
        let response = check_guess("whale", 10000000);
        assert_eq!(response, Err("Word not found".to_string()));
        Ok(())
    }
}