use rust50::get_string;
use std::env::args;
use std::process::exit;

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() != 2 {
        println!("Usage: ./substitution key");
        exit(1);
    }
    let key = &argv[1];
    if !is_valid(&key) {
        exit(1);
    }
    let ptext = get_string("plaintext: ");
    print!("ciphertext: ");
    for i in ptext.chars() {
        print!("{}", rotate(i, key))
    }
    println!();
    exit(0);
}

fn rotate(c: char, key: &String) -> char {
    if c.is_alphabetic() {
        if c.is_uppercase() {
            return key.chars().nth((c as u8 - b'A') as usize).unwrap();
        } else {
            return key.chars().nth((c as u8 - b'a') as usize).unwrap().to_ascii_lowercase();
        }
    } else {
        return c;
    }
}

fn is_valid(key: &String) -> bool {
    if key.len() == 26 {
        for i in key.chars() {
            if !i.is_alphabetic() {
                println!("Key must contain 26 alphabetic characters.");
                return false;
            }
        }
        for i in 0..key.len() - 1 {
            for j in i+1..key.len() {
                if key.chars().nth(i).unwrap() == key.chars().nth(j).unwrap() {
                    println!("No Duplicates");
                    return false;
                }
            }
        }
        return true;
    } else {
        println!("Key must contain 26 characters.");
        return false;
    }
}
