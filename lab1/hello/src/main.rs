use std::io::{self, Write};

fn main() {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed To Read Input");
    println!("Hello {name}");
}
