use rust50::{get_int, get_string};
use std::env::args;
use std::process::exit;

struct Candidate {
    name: String,
    votes: i32,
}

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() < 2 {
        eprintln!("Usage: plurality [candidate ...]");
        exit(1);
    }

    // Populate array of candidates
    let candidate_count = argv.len() - 1;
    if candidate_count > 9 {
        eprintln!("Maximum number of candidates is 9");
        exit(2);
    }
    let mut candidates : Vec<Candidate> = argv.into_iter()
    .skip(1).map(|  name | Candidate { name , votes : 0}).collect();
    let voter_count = get_int("Number of voter: ");
    for _i in 0..voter_count {
        let name = get_string("Vote: ");
        if !vote(&name, &mut candidates) {
            println!("Invalid vote.");
        }
    }
    let max = candidates.iter().map(|c| c.votes).max().unwrap_or_default();
    print_winner(&mut candidates, max)
}

fn print_winner(canidates: &mut Vec<Candidate>, max: i32) {
    for i in canidates {
        if i.votes == max {
            println!("{}", i.name);
        }
    }
}

fn vote(name: &String, canidates: &mut Vec<Candidate>) -> bool {
    for i in canidates {
        if i.name == *name {
            i.votes += 1;
            return true;
        }
    }
    return false;
}
