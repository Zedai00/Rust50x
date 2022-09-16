use std::io::{self, Write};

const QUARTER: u32 = 25;
const DIME: u32 = 10;
const NICKEL: u32 = 5;
const PENNIE: u32 = 1;

fn main() {
    let mut cents = get_cents();
    let mut coins = 0;
    for i in [QUARTER, DIME, NICKEL, PENNIE] {
        coins += calculate_coins(&mut cents, i)
    }
    println!("{coins}");
}

fn calculate_coins(cents: &mut u32, denomination: u32) -> u32 {
    let result = *cents / denomination;
    *cents %= denomination;
    result
}

fn get_cents() -> u32 {
    loop {
        print!("Change Owed: ");
        io::stdout().flush().unwrap();
        let mut cents = String::new();
        io::stdin()
            .read_line(&mut cents)
            .expect("Failed To Read Input");
        match cents.trim().parse::<u32>() {
            Ok(c) => return c,
            _ => continue,
        }
    }
}
