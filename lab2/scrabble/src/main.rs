use std::io::{self, Write};

const POINTS: [i32; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

fn main() {
    let (word1, word2) = (
        compute_score(&get_input("1")),
        compute_score(&get_input("2")),
    );
    if word1 > word2 {
        println!("Player 1 Wins!");
    } else if word1 < word2 {
        println!("Player 2 Wins!");
    } else {
        println!("Tie!")
    }
}

fn compute_score(word: &str) -> i32 {
    word.chars()
        .filter(char::is_ascii_alphabetic)
        .map(|c| POINTS[c.to_ascii_lowercase() as usize - 'a' as usize])
        .sum()
}

fn get_input(player: &str) -> String {
    print!("Player {player}: ");
    io::stdout().flush().unwrap();
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed To Read Input");
    word
}
