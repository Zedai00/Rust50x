use std::{
    env,
    io::{self, Write},
    process,
};

const MAX: i32 = 9;

struct Candidate {
    name: String,
    votes: i32,
}

struct Election {
    candidates: Vec<Candidate>,
}

impl Election {
    fn new(args: Vec<String>) -> Self {
        Self {
            candidates: args
                .into_iter()
                .map(|name| Candidate { name, votes: 0 })
                .collect(),
        }
    }

    fn vote(&mut self, voter_count: i32) {
        for _ in 0..voter_count {
            let name = get_input("Vote: ");

            if let Some(candidate) = self.candidates.iter_mut().find(|c| c.name == name) {
                candidate.votes += 1;
            } else {
                println!("Invalid Vote")
            }
        }
    }

    fn get_winner(&mut self) -> Vec<&Candidate> {
        let winner = self
            .candidates
            .iter()
            .max_by_key(|c| c.votes)
            .expect("No Votes Found");
        self.candidates
            .iter()
            .filter(|c| c.votes == winner.votes)
            .collect()
    }
}

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("Usage: plurality [candidate ...]");
        process::exit(1);
    }

    let candidate_count = args.len() - 1;
    if candidate_count > MAX as usize {
        println!("Maximum number of candidates is {MAX}");
        process::exit(2);
    }

    let mut election = Election::new(args.skip(1).collect());
    let voter_count = match get_input("Number of voters: ").parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please Input A Valid Number");
            process::exit(3);
        }
    };

    Election::vote(&mut election, voter_count);
    let winners = Election::get_winner(&mut election);
    for i in winners {
        println!("{}", i.name)
    }
}

fn get_input(text: &str) -> String {
    print!("{text}");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed To Read Input");
    String::from(text.trim())
}
