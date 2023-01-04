use std::fs;
use std::error::Error;
use rand::prelude::*;
use std::io;

pub fn check_word(word: &String, input_word: &String) -> String {
    let mut temp_word = String::new(); 
    let mut letter_count: u8 = 0;
    for (idxw, w) in word.chars().enumerate() {
        for (idxi, i) in input_word.chars().enumerate() {
            if i == w && idxi == idxw {
                temp_word.push_str("o");
                letter_count += 1;
            } else if idxi == idxw {
                temp_word.push_str("-");
            } else if i == w {
                letter_count += 1;
            }
        }
    }
    temp_word = format!("{}     {} correct letters.", temp_word, letter_count);
    temp_word
}

pub fn check_letter(word: &String, letter: &String) -> bool {
    word.contains(letter) 
}

pub fn get_user_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    Ok(input)
}

pub fn read_words_file() -> Result<(Vec<String>, u64), Box<dyn Error>> {
    let words = fs::read_to_string("words.txt")?;
    let mut word_vec: Vec<String> = Vec::new();
    let mut num: u64 = 0;
    let lines = words.lines();
    for line in lines {
        word_vec.push(line.to_string());
        num += 1;
    }
    
    Ok((word_vec, num))
}

pub fn select_random_word(words: &Vec<String>, qty: &u64) -> String {
    let mut rng = rand::thread_rng();
    let rand_index: u64 = rng.gen_range(0..qty - 1);
    words[rand_index as usize].clone()
}

pub mod utils {
    use regex::Regex;
    use std::error::Error;
    use std::{fs, io::Write};

    pub fn _make_word_file(letters: u32) -> Result<(), Box<dyn Error>> {
        let book = fs::read_to_string("don-quijote.txt").expect("problem");
        let mut strings = String::new();
        let re = Regex::new(&format!("^[A-Za-z]{{{}}}$", letters.to_string())[..]).unwrap();
        for s in book.split_whitespace() {
            if re.is_match(s) {
                let a = &format!("\n{}", s.to_lowercase())[..];
                if !strings.contains(a) {
                    strings.push_str(a);
                }
            }
        }
        let mut f = fs::File::create("words.txt").expect("unable to create file");
        f.write_all(strings.as_bytes()).expect("Could not write");
        Ok(())
    }
}
