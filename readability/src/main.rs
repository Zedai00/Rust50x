use rust50::get_string;

fn main() {
    let text = get_string("Text: ");
    let letters = count_letters(&text);
    let words = count_words(&text);
    let sentences = count_sentences(&text);
    
    let l = letters as f32 / words as f32 * 100.0;
    let s = sentences as f32 / words as f32 * 100.0;
    let index = 0.0588 * l - 0.296 * s - 15.8;
    let index = index.round() as i32;

    if index < 1 {
        println!("Before Grade 1");
    } else if index >= 16 {
        println!("Grade 16+");
    } else {
        println!{"Grade {}", index};
    }
}

fn count_sentences(text: &String) -> i32 {
    let mut count = 0;
    for i in text.chars() {
        if i == '.' || i == '?' || i == '!' {
            count += 1
        }
    }
    count
}

fn count_words(text: &String) -> i32 {
    let mut count = 0;
    for i in text.chars() {
        if i == ' ' {
            count += 1
        }
    }
    count += 1;
    count
}

fn count_letters(text: &String) -> i32 {
    let mut count = 0;
    for i in text.chars() {
        if i.is_alphabetic() {
            count += 1
        }
    }
    count
}