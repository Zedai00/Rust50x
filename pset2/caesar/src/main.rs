use std::{
    env::args,
    io::{self, Write},
    process,
};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: caesar key");
        process::exit(1);
    }
    let key = match args[1].parse::<u32>() {
        Ok(k) => k,
        Err(_) => {
            println!("Usage: caesar key");
            process::exit(1);
        }
    };

    let ptext = get_ptext();
    println!("cipertext: {}", rotate(ptext, key));
}

fn rotate(ptext: String, key: u32) -> String {
    ptext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u32;
                (((c as u32 - base + key) % 26) + base) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

fn get_ptext() -> String {
    print!("plaintext: ");
    io::stdout().flush().unwrap();
    let mut ptext = String::new();
    io::stdin()
        .read_line(&mut ptext)
        .expect("Failed To Read Input.");
    ptext
}
