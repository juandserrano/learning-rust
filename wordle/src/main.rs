use std::process;
use wordle::*;

fn main() {
    let (words, qty) = read_words_file().unwrap_or_else(|err| {
        eprintln!("Problem reading word file: {}", err);
        process::exit(1);
    });

    let word: String = select_random_word(&words, &qty);
    println!("{}", word);

    let input = get_user_input().unwrap_or_else(|err| {
        eprintln!("Problem getting user input: {}", err);
        process::exit(1);
    });
    println!("{}", check_word(&word, &input));

}
