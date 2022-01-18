use rust50::get_input;
use std::env::args;
use std::process::exit;
use std::vec;

const MAX: usize = 9;

#[derive(Clone)]
struct Pair {
    winner: i32,
    loser: i32,
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Usage: runoff [candidate ...]");
        exit(1);
    }

    let candidate_count = args.len() - 1;
    if candidate_count > MAX {
        eprintln!("Maximum number of candidates is {}", MAX);
        exit(2);
    }

    let candidates: Vec<String> = args.into_iter().skip(1).collect();

    let mut preferences = vec![vec![0 as usize; MAX]; MAX];
    let mut locked = vec![vec![false; MAX]; MAX];
    let mut pair_count = 0;
    let voter_count: i32 = get_input("Number of voters: ");
    for _i in 0..voter_count {
        let mut ranks = vec![0; candidate_count];
        for j in 0..candidate_count {
            let name: String = get_input(&format!("Rank {}: ", j + 1));
            if !vote(j, name, &mut ranks, &candidates) {
                eprintln!("Invalid vote.");
                exit(3);
            }
        }

        record_preferences(&ranks, &mut preferences, candidate_count);
        println!();
    }
    let count = MAX * (MAX - 1) / 2;
    let mut pairs: Vec<Pair> = vec![
        Pair {
            winner: 0,
            loser: 0
        };
        count
    ];
    add_pairs(&mut pairs, &preferences, candidate_count, &mut pair_count);
    sort_pairs(&mut pair_count, &preferences, &mut pairs);
    lock_pairs(&mut pair_count, &mut pairs, &mut locked, candidate_count);
    print_winner(candidate_count, &mut locked, &candidates);
}

fn print_winner(candidate_count: usize, locked: &mut Vec<Vec<bool>>, candidates: &Vec<String>) {
    for i in 0..candidate_count {
        for j in 0..candidate_count {
            if locked[j][i] == true {
                println!("{}", candidates[j]);
                return;
            }
        }
    }
    return;
}

fn lock_pairs(
    pair_count: &mut usize,
    pairs: &mut Vec<Pair>,
    locked: &mut Vec<Vec<bool>>,
    canditate_count: usize,
) {
    for i in 0..*pair_count {
        if cycle(pairs[i].winner, pairs[i].loser, canditate_count, locked) == false {
            locked[pairs[i].winner as usize][pairs[i].loser as usize] = true;
        }
    }
}

fn cycle(winner: i32, loser: i32, candidate_count: usize, locked: &mut Vec<Vec<bool>>) -> bool {
    for i in 0..candidate_count {
        if locked[winner as usize][loser as usize] == true {
            if loser == i as i32 {
                return true;
            }

            if cycle(loser, i as i32, candidate_count, locked) == true {
                return true;
            }
        }
    }
    return false;
}

fn sort_pairs(pair_count: &mut usize, preferences: &Vec<Vec<usize>>, pairs: &mut Vec<Pair>) {
    for i in 0..*pair_count {
        for j in 0..*pair_count {
            if preferences[pairs[i].winner as usize][pairs[j].loser as usize]
                > preferences[pairs[j].winner as usize][pairs[i].loser as usize]
            {
                let temp1 = pairs[i].winner;
                let temp2 = pairs[j].winner;

                let temp3 = pairs[i].loser;
                let temp4 = pairs[j].loser;

                pairs[i].winner = temp2;
                pairs[j].winner = temp1;

                pairs[i].loser = temp4;
                pairs[j].loser = temp3;
            }
        }
    }
}
fn add_pairs(
    pairs: &mut Vec<Pair>,
    preferences: &Vec<Vec<usize>>,
    candidate_count: usize,
    pair_count: &mut usize,
) {
    for i in 0..candidate_count {
        for j in 0..candidate_count {
            if preferences[i][j] > preferences[j][i] {
                pairs[*pair_count as usize].winner = i as i32;
                pairs[*pair_count as usize].loser = j as i32;
                *pair_count += 1;
            }
        }
    }
}

fn record_preferences(ranks: &Vec<i32>, preferences: &mut Vec<Vec<usize>>, canditate_count: usize) {
    for i in 0..canditate_count {
        for j in i + 1..canditate_count {
            preferences[ranks[i] as usize][ranks[j] as usize] += 1;
        }
    }
}

fn vote(rank: usize, name: String, ranks: &mut Vec<i32>, candidates: &Vec<String>) -> bool {
    for i in 0..candidates.len() {
        if candidates[i] == name {
            ranks[rank] = i as i32;
            return true;
        }
    }
    return false;
}
