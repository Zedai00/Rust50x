use std::io::{self, Write};

fn main() {
    // Example number visa
    // let n: usize = 4003600000000014;
    let num = get_card();
    let num = number_to_vec(num);
    println!("{}", card(&num, luhn(&num)));
}

fn card(num: &Vec<u32>, chksum: u32) -> &str {
    if chksum.to_string().chars().last().unwrap() == '0' {
        let digits = num.len();
        let first_nums = format!("{}{}", num[0], num[1]);
        if (digits == 13 || digits == 16) && first_nums.chars().next().unwrap() == '4' {
            return "VISA";
        } else if digits == 15 && (first_nums == "34" || first_nums == "37") {
            return "AMEX";
        } else if digits == 16 && (51..=55).any(|n| first_nums.contains(&n.to_string())) {
            return "MASTERCARD";
        } else {
            return "INVALID";
        }
    } else {
        return "INVALID";
    }
}

fn luhn(num: &Vec<u32>) -> u32 {
    let mut even = 0;
    for i in num.iter().rev().skip(1).step_by(2) {
        let a = number_to_vec((*i * 2) as usize).iter().sum::<u32>();
        even += a
    }
    let mut odd = 0;
    for i in num.iter().rev().step_by(2) {
        let a = number_to_vec((*i) as usize).iter().sum::<u32>();
        odd += a
    }
    even + odd
}

fn number_to_vec(n: usize) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn get_card() -> usize {
    loop {
        print!("Number: ");
        io::stdout().flush().unwrap();
        let mut number = String::new();
        io::stdin().read_line(&mut number).unwrap();
        match number.trim().parse() {
            Ok(n) => return n,
            _ => continue,
        }
    }
}
