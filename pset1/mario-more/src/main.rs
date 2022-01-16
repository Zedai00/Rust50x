use rust50::get_int;

fn main() {
    let mut height;
    loop {
        height = get_int("Height: ");
        if height >= 1 && height <= 8 {
            break;
        }
    }
    for _i in 0..height {
        for _j in 0..height - 1 {
            print!(" ");
        }
        for _k in 0.._i + 1 {
            print!("#");
        }
        print!("  ");
        for _l in 0.._i + 1 {
            print!("#");
        }
        height -= 1;
        println!();
    }
}
