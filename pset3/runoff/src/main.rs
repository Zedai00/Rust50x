use std::{
    env::args,
    io::{self, Write},
    process::exit,
};

#[derive(Debug)]
struct Candidate {
    name: String,
    votes: i32,
    eliminated: bool,
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: runoff [candidate ...]");
        exit(1);
    }
    let candidate_count = (args.len() - 1) as i32;
    if candidate_count > 9 {
        println!("Maximum number of candidates is 9");
        exit(2);
    }
    let mut candidates = args
        .into_iter()
        .skip(1)
        .map(|n| Candidate {
            name: n,
            votes: 0,
            eliminated: false,
        })
        .collect::<Vec<_>>();
    let voter_count = match get_input("Number of voters: ").parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a Number");
            exit(3)
        }
    };
    if voter_count < 100 {
        println!("Maximum number of voters is 100");
        exit(4)
    };

    let mut preferences: Vec<Vec<usize>> = vec![vec![]];

    for i in 0..voter_count {
        for j in 0..candidate_count {
            let name = get_input(&format!("Rank {}", j + 1));

            if !vote(i, j, name, &mut candidates, &mut preferences) {
                println!("Invalid Vote.");
                exit(4)
            }
        }
    }

    loop {
        tabulate();

        let won = print_winner();
        if won {
            break;
        }

        let min = find_min();
        let tie = is_tie(min);

        if tie {
            for i in &candidates {
                if !i.eliminated {
                    println!("{}", i.name);
                }
            }
            break;
        }

        eliminate(min);

        for i in &mut candidates {
            i.votes = 0;
        }
    }
}

fn vote(
    voter: i32,
    rank: i32,
    name: String,
    candidates: &mut [Candidate],
    preferences: &mut [Vec<usize>],
) -> bool {
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.name == name {
            preferences[voter as usize][rank as usize] = index;
            return true;
        }
    }
    false
}

fn tabulate() {}

fn print_winner() -> bool {
    false
}

fn find_min() -> i32 {
    0
}

fn is_tie(min: i32) -> bool {
    false
}

fn eliminate(min: i32) {}

fn get_input(text: &str) -> String {
    print!("{text}");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed To Read Line");
    text
}
