use rust50::{get_int, get_string};
use std::env::args;
use std::process::exit;
use std::vec;

// Max voters and candidates
const MAX_VOTERS: usize = 100;
const MAX_CANDIDATES: usize = 9;

// Candidates have name, vote count, eliminated status
struct Candidate {
    name: String,
    votes: i32,
    eliminated: bool,
}
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Usage: runoff [candidate ...]");
        exit(1);
    }

    let candidate_count = args.len() - 1;
    if candidate_count > MAX_CANDIDATES {
        eprintln!("Maximum number of candidates is {}", MAX_CANDIDATES);
        exit(2);
    }

    let mut candidates: Vec<Candidate> = args
        .into_iter()
        .skip(1)
        .map(|name| Candidate {
            name,
            votes: 0,
            eliminated: false,
        })
        .collect();

    let voter_count = get_int("Number of voters: ");
    if voter_count > MAX_VOTERS as i32 {
        eprintln!("Maximum number of voters is {}", MAX_VOTERS);
        exit(3);
    }

    let mut preferences = vec![vec![String::from("_"); MAX_CANDIDATES]; MAX_VOTERS];
    for i in 0..voter_count {
        for j in 0..candidate_count {
            let name = get_string(&format! {"Rank {}", j+1});
            if !vote(i, j, name, &mut preferences, &mut candidates) {
                eprintln!("Invalid vote.");
                exit(4);
            }
        }
    }

    loop {
        tabulate(&mut candidates, &preferences, voter_count, candidate_count);
        let won = print_winner(&candidates, voter_count);
        if won {
            break;
        }
        let min = candidates.iter().map(|c| c.votes).min().unwrap_or_default();
        let tie = is_tie(&candidates, min);
        if tie {
            for i in candidates {
                if !i.eliminated {
                    println!("{}", i.name);
                }
            }
            break;
        }
        eliminate(&mut candidates, min);
        for i in &mut candidates {
            i.votes = 0
        }
    }
    exit(0);
}

fn eliminate(candidates: &mut Vec<Candidate>, min: i32) {
    for i in candidates {
        if i.votes == min {
            i.eliminated = true;
        }
    }
}

fn is_tie(candidates: &Vec<Candidate>, min: i32) -> bool {
    for i in candidates {
        if i.votes != min {
            return false;
        }
    }
    return true;
}

fn print_winner(candidates: &Vec<Candidate>, voter_count: i32) -> bool {
    for i in candidates {
        if i.votes > (voter_count / 2) {
            println!("{}", i.name);
            return true;
        }
    }
    return false;
}

fn tabulate(
    candidates: &mut Vec<Candidate>,
    prefer: &Vec<Vec<String>>,
    voter_count: i32,
    candidate_count: usize,
) {
    for i in candidates {
        if !i.eliminated {
            for j in 0..voter_count as usize {
                for k in 0..candidate_count {
                    if i.name == prefer[j][k] {
                        i.votes += 1;
                        break;
                    }
                }
            }
        }
    }
}

fn vote(
    voter: i32,
    rank: usize,
    name: String,
    prefer: &mut Vec<Vec<String>>,
    candidates: &mut Vec<Candidate>,
) -> bool {
    for i in candidates {
        if i.name == name {
            prefer[voter as usize][rank] = name;
            return true;
        }
    }
    return false;
}
