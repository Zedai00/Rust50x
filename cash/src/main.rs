use rust50::get_int;

fn main() {
    // Ask how many cents the customer is owed
    let mut cents = get_cents();
    
    // Calculate the number of quarters to give the customer
    let quarters = calculate_quarters(cents);
    cents = cents - quarters * 25;

    // Calculate the number of dimes to give the customer
    let dimes = calculate_dimes(cents);
    cents = cents - dimes * 10;

    // Calculate the number of nickels to give the customer
    let nickels = calculate_nickels(cents);
    cents = cents - nickels * 5;

    // Calculate the number of pennies to give the customer
    let pennies = calculate_pennies(cents);
    // cents = cents - pennies * 1;

    // Sum coins
    let coins = quarters + dimes + nickels + pennies;

    // Print total number of coins to give the customer
    println!("{}", coins);
}

fn calculate_pennies(mut cents: i64) -> i64 {
    let mut count = 0;
    loop {
        if cents >= 1 {
            cents -= 1;
            count += 1;
        } else {
            break
        }
    }
    count
}

fn calculate_nickels(mut cents: i64) -> i64 {
    let mut count = 0;
    loop {
        if cents >= 5 {
            cents -= 5;
            count += 1;
        } else {
            break
        }
    }
    count
}

fn calculate_dimes(mut cents: i64) -> i64 {
    let mut count = 0;
    loop {
        if cents >= 10 {
            cents -= 10;
            count += 1;
        } else {
            break
        }
    }
    count
}

fn calculate_quarters(mut cents: i64) -> i64 {
    let mut count = 0;
    loop {
        if cents >= 25 {
            cents -= 25;
            count += 1;
        } else {
            break
        }
    }
    count
}

fn get_cents() -> i64 {
    let mut cents;
    loop {
        cents = get_int("Cents: ");
        if cents >= 0 { break }
    }
    cents
}
