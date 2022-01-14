use rust50::get_int;

fn main() {
    let card = get_int("Card: ");
    let card: Vec<u32> = card
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut sum: u32 = 0;
    let mut card_array = card.clone();
    for i in card_array.iter_mut().rev().skip(1).step_by(2) {
        *i *= 2;
    }
    for i in card_array.iter_mut() {
        let j = *i % 10;
        let k = *i / 10 % 10;
        sum += j;
        sum += k;
    }
    if sum % 10 == 0 {
        let first = card[0].to_string() + &card[1].to_string();
        let first: i32 = first.parse().unwrap();
        if (card_array.len() == 13 || card_array.len() == 16) && card[0] == 4 {
            println!("VISA");
        } else if card_array.len() == 15 && (first == 34 || first == 37) {
            println!("AMEX")
        } else if card_array.len() == 16
            && (first == 51 || first == 52 || first == 53 || first == 54 || first == 55)
        {
            println!("MASTERCARD");
        } else {
            println!("INVALID");
        }
    } else {
        println!("INVALID");
    }
}
