use rand::{thread_rng, Rng};
use chrono::{DateTime, Local, TimeZone};

static LINES: &str = include_str!("words.txt");
static ANSWERS: &str = include_str!("answers.txt");

pub fn get_word(index: usize) -> Option<&'static str> {
    LINES.lines().nth(index)
}

pub fn is_valid_word(word: &str) -> bool {
    LINES.lines().any(|dict_word| dict_word == word)
}

pub fn get_random_word() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..LINES.lines().count())
}

pub fn get_word_index_for_date(date: DateTime<Local>) -> Option<usize> {
    let origin = Local.ymd(2021, 6, 18).and_hms(0, 0, 0);
    let offset = date.signed_duration_since(origin);
    let offset_days = offset.num_days();

    dbg!(&offset_days);

    let answer = ANSWERS.lines().nth(offset_days.try_into().ok()?)?;

    LINES.lines().position(|word| word == answer)
}