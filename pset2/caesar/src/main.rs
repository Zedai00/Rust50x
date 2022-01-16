use rust50::get_string;
use std::char;
use std::env::args;
use std::process::exit;

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() != 2 {
        println!("Usage: ./caesar key");
        exit(1);
    }
    if !only_digits(&argv[1]) {
        println!("Usage: ./caesar key");
        exit(1);
    }
    let key: i32 = argv[1].parse().unwrap();
    let ptext = get_string("plaintext: ");
    print!("ciphertext: ");
    for i in ptext.chars() {
        print!("{}", rotate(i, key))
    }
    println!();
    exit(0);
}

fn rotate(c: char, key: i32) -> char {
    if c.is_ascii_alphabetic() {
        if c.is_ascii_uppercase() {
            ((((c as u8 - b'A') + key as u8) % 26) + b'A') as char
        } else {
            ((((c as u8 - b'a') + key as u8) % 26) + b'a') as char
        }
    } else {
        c
    }
}

fn only_digits(s: &String) -> bool {
    for i in s.chars() {
        if !i.is_digit(10) {
        return false;
        }
    }
    true
}
