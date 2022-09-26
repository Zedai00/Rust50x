use std::io::{self, Write};

fn main() {
    let text = get_input();

    let letters = count_letters(&text);
    let words = count_words(&text);
    let sentences = count_sentences(&text);

    let average_letters = (letters / words) * 100f64;
    let average_sentences = (sentences / words) * 100f64;

    let index = (0.0588 * average_letters - 0.296 * average_sentences - 15.8).round();
    if index < 1f64 {
        println!("Before Grade 1");
    } else if index <= 16f64 {
        println!("Grade {index}");
    } else {
        println!("Grade 16+");
    }
}

fn count_sentences(text: &str) -> f64 {
    text.chars().filter(|c| ['.', '!', '?'].contains(c)).count() as f64
}

fn count_words(text: &str) -> f64 {
    (text.chars().filter(char::is_ascii_whitespace).count() + 1) as f64
}

fn count_letters(text: &str) -> f64 {
    text.chars().filter(char::is_ascii_alphabetic).count() as f64
}

fn get_input() -> String {
    print!("Text: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed To Read Input.");
    String::from(text.trim())
}
