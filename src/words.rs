use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::{thread_rng, Rng};

pub fn get_word(index: u32) -> Result<String, String> {
    let mut counter = 0;
    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if index == counter {
                if let Ok(i) = line {
                    return Ok(i);
                } else {
                    return Err("Unable to load wordfile".to_string());
                }
            }
            counter += 1;
        }
        return Err("Index greater than wordlist length".to_string());
    }
    return Err("Unable to load wordfile".to_string());
}

pub fn get_random_word() -> Result<u32, String> {
    let mut rng = thread_rng();
    let lines = read_lines("./words.txt").map_err(|_| "Unable to load wordfile".to_string())?;
    Ok(rng.gen_range(0..lines.count()).try_into().map_err(|_| "Failed to generate word")?)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}