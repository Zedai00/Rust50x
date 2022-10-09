use std::{
    env::args,
    io::{self, Write},
    process::ExitCode,
};

struct Candidate {
    name: String,
    votes: i32,
    eliminated: bool,
}

struct Election {
    candidates: Vec<Candidate>,
    voters: i32,
    preferences: Vec<Vec<usize>>,
}

impl Election {
    fn new(candidates: Vec<String>, voters: i32) -> Self {
        Self {
            preferences: vec![vec![0; candidates.len() as usize]; voters as usize],
            candidates: candidates
                .into_iter()
                .map(|n| Candidate {
                    name: n,
                    votes: 0,
                    eliminated: false,
                })
                .collect::<Vec<Candidate>>(),
            voters,
        }
    }

    fn vote(&mut self, voter: i32, rank: usize, name: String) -> bool {
        for (index, candidate) in self.candidates.iter().enumerate() {
            if candidate.name == name {
                self.preferences[voter as usize][rank] = index;
                return true;
            }
        }
        false
    }

    fn commence(&mut self) -> Result<Vec<String>, ExitCode> {
        for voter in 0..self.voters {
            for rank in 0..self.candidates.len() {
                let name = Self::get_input(&format!("Rank {}: ", rank + 1));
                if !Self::vote(self, voter, rank, name) {
                    println!("Invalid Vote.");
                    return Err(4.into());
                }
            }
            println!();
        }
        let mut winners: Vec<String> = Vec::new();
        loop {
            self.tabulate();

            if let Ok(candidate) = self.won() {
                return Ok(vec![candidate]);
            }

            let min = self.find_min();
            let tie = self.is_tie(min);

            if tie {
                for i in &mut self.candidates {
                    if !i.eliminated {
                        winners.push(i.name.to_owned())
                    }
                }
                return Ok(winners);
            }

            self.eliminate(min);

            for i in &mut self.candidates {
                i.votes = 0;
            }
        }
    }

    fn tabulate(&mut self) {
        for i in 0..self.voters {
            for j in 0..self.candidates.len() {
                if !self.candidates[self.preferences[i as usize][j]].eliminated {
                    self.candidates[self.preferences[i as usize][j]].votes += 1;
                    break;
                }
            }
        }
    }

    fn won(&self) -> Result<String, bool> {
        for i in &self.candidates {
            if i.votes > (self.voters / 2) {
                return Ok(i.name.clone());
            }
        }
        Err(false)
    }

    fn find_min(&mut self) -> i32 {
        self.candidates
            .iter()
            .filter(|c| !c.eliminated)
            .min_by_key(|c| c.votes)
            .unwrap()
            .votes
    }

    fn is_tie(&self, min: i32) -> bool {
        for i in &self.candidates {
            if !i.eliminated && i.votes != min {
                return false;
            }
        }
        true
    }

    fn eliminate(&mut self, min: i32) {
        self.candidates
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
        text.trim().to_string()
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: runoff [candidate ...]");
        return 1.into();
    } else if args.len() > 9 {
        println!("Maximum number of candidates is 9");
        return 2.into();
    }

    let voters = match Election::get_input("Number of voters: ").parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a Valid Input");
            println!("Input Should A Number Between 1 And 100");
            return 3.into();
        }
    };

    let mut election = Election::new(args, voters);

    match election.commence() {
        Ok(winners) => {
            for i in winners {
                println!("{i}");
            }
            0.into()
        }
        Err(c) => c,
    }
}
