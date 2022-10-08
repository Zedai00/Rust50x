use std::{
    env::args,
    io::{self, Write},
    process::ExitCode,
};

#[derive(Debug)]
struct Candidate {
    name: String,
    votes: i32,
    eliminated: bool,
}

struct Election {
    candidates: Vec<Candidate>,
}

impl Election {
    fn new(candidates: Vec<String>) -> Self {
        Election {
            candidates: candidates
                .into_iter()
                .map(|n| Candidate {
                    name: n,
                    votes: 0,
                    eliminated: false,
                })
                .collect::<Vec<Candidate>>(),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 2 {
        println!("Usage: runoff [candidate ...]");
        return 1.into();
    }
    let candidate_count = (args.len() - 1) as i32;
    if candidate_count > 9 {
        println!("Maximum number of candidates is 9");
        return 2.into();
    }
    // let mut candidates = args
    //     .into_iter()
    //     .skip(1)
    //     .map(|n| Candidate {
    //         name: n,
    //         votes: 0,
    //         eliminated: false,
    //     })
    //     .collect::<Vec<_>>();
    let mut candidates = Election::new(args);
    let voter_count = match get_input("Number of voters: ").trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a Number");
            return 3.into();
        }
    };
    if voter_count > 100 {
        println!("Maximum number of voters is 100");
        return 4.into();
    };

    let mut preferences = vec![vec![0; candidate_count as usize]; voter_count as usize];

    for i in 0..voter_count {
        for j in 0..candidate_count {
            let name = get_input(&format!("Rank {}: ", j + 1)).trim().to_string();

            if !vote(i, j, name, &mut candidates, &mut preferences) {
                println!("Invalid Vote.");
                return 4.into();
            }
        }
        println!();
    }

    loop {
        tabulate(voter_count, &mut candidates, &mut preferences);

        let won = print_winner(&mut candidates, voter_count);
        if won {
            return 0.into();
        }

        let min = find_min(&mut candidates);
        let tie = is_tie(min, &mut candidates);

        if tie {
            for i in &candidates {
                if !i.eliminated {
                    println!("{}", i.name);
                }
            }
            return 0.into();
        }

        eliminate(min, &mut candidates);

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
    preferences: &mut Vec<Vec<usize>>,
) -> bool {
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.name == name {
            preferences[voter as usize][rank as usize] = index;
            return true;
        }
    }
    false
}

fn tabulate(voter_count: i32, candidates: &mut [Candidate], preferences: &mut [Vec<usize>]) {
    for i in 0..voter_count {
        for j in 0..candidates.len() {
            if !candidates[preferences[i as usize][j]].eliminated {
                candidates[preferences[i as usize][j]].votes += 1;
                break;
            }
        }
    }
}

fn print_winner(candidates: &mut [Candidate], voter_count: i32) -> bool {
    for i in candidates {
        if i.votes > (voter_count / 2) {
            println!("Winner: {}", i.name);
            return true;
        }
    }
    false
}

fn find_min(candidates: &mut [Candidate]) -> i32 {
    candidates
        .iter()
        .filter(|c| !c.eliminated)
        .min_by_key(|c| c.votes)
        .unwrap()
        .votes
}

fn is_tie(min: i32, candidates: &mut [Candidate]) -> bool {
    for i in candidates {
        if !i.eliminated && i.votes != min {
            return false;
        }
    }
    true
}

fn eliminate(min: i32, candidates: &mut [Candidate]) {
    candidates
        .iter_mut()
        .filter(|c| c.votes == min)
        .for_each(|c| c.eliminated = true);
}

fn get_input(text: &str) -> String {
    print!("{text}");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed To Read Line");
    text
}
