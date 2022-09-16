use std::io::{self, Write};

fn main() {
    // Example number visa
    // let n: usize = 4003600000000014;
    let num = get_card();
    let num = number_to_vec(num);
    let mut chksum = 0;
    for i in ["even", "odd"].iter() {
        chksum += luhn(&num, i)
    }
    println!("{}", card(&num, chksum));
}

fn card(num: &Vec<u32>, chksum: u32) -> String {
    if chksum.to_string().chars().last().unwrap() == '0' {
        let digits = num.len();
        let first_nums = format!("{}{}", num[0], num[1]);
        if (digits == 13 || digits == 16) && first_nums.chars().next().unwrap() == '4' {
            return String::from("VISA");
        } else if digits == 15 && (first_nums == "34" || first_nums == "37") {
            return String::from("AMEX");
        } else if digits == 16 && (51..=55).any(|n| first_nums.contains(&n.to_string())) {
            return String::from("MASTERCARD");
        } else {
            return String::from("INVALID");
        }
    } else {
        return String::from("INVALID");
    }
}

fn luhn(num: &Vec<u32>, mode: &str) -> u32 {
    let mut s = 0;
    let mut mult = 0;
    match mode {
        "odd" => {
            s = 0;
            mult = 1;
        }
        "even" => {
            s = 1;
            mult = 2;
        }
        _ => panic!("Unkown Mode"),
    }
    let mut result = 0;
    for i in num.iter().rev().skip(s).step_by(2) {
        let a = number_to_vec((*i * mult) as usize).iter().sum::<u32>();
        result += a
    }
    result
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
