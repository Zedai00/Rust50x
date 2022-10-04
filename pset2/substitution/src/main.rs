use std::{
    collections::HashSet,
    env::args,
    io::{self, Write},
    iter::FromIterator,
    process,
};

fn main() {
    let args: Vec<String> = args().collect();
    if !is_alphabets(&args[1]) {
        println!("Key Must Contain 26 Alphabets Characters");
        process::exit(1);
    }
    let key: Vec<_> = args[1].trim().chars().collect();
    let ptext = get_input();
    println!("ciphertext: {}", substitute(ptext, key))
}

fn substitute(ptext: String, key: Vec<char>) -> String {
    ptext
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    key[(c as usize - 'A' as usize) % 26]
                } else {
                    key[(c as usize - 'a' as usize) % 26].to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

fn get_input() -> String {
    print!("plaintext: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed To Read Input");
    text
}

fn is_alphabets(text: &str) -> bool {
    text.len() == 26 && HashSet::<char>::from_iter('A'..='Z') == text.chars().collect()
}
