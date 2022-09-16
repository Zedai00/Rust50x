use std::{
    io::{self, Write},
    iter,
};

fn main() {
    let h = get_height();
    for i in 0..h {
        let j = h - (i + 1);
        let k = h - j;
        let line = iter::repeat(" ")
            .take(j)
            .chain(iter::repeat("#").take(k))
            .collect::<String>();
        println!("{}", line)
    }
}

fn get_height() -> usize {
    loop {
        let mut height = String::new();
        print!("Height: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read input");
        if let Ok(i @ 1..=8) = height.trim().parse() {
            return i;
        }
    }
}
