use rand::{thread_rng, Rng};

static LINES: &str = include_str!("words.txt");

pub fn get_word(index: usize) -> Option<&'static str> {
    LINES.lines().nth(index)
}

pub fn is_valid_word(word: &str) -> bool {
    LINES.lines().any(|dict_word| dict_word == word)
}

pub fn get_random_word() -> Result<usize, String> {
    let mut rng = thread_rng();
    Ok(rng.gen_range(0..LINES.lines().count()))
}