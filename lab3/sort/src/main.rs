use std::{fs, process, time};

fn main() {
    let dir = String::from("sort");
    for i in fs::read_dir(dir).unwrap() {
        let mut cmd = process::Command::new(i.unwrap().path().to_str().unwrap());
        for j in fs::read_dir("text").unwrap() {
            let now = time::Instant::now();
            cmd.arg(j.unwrap().path().to_str().unwrap())
                .output()
                .expect("Failed");
            println!("{}", now.elapsed().as_micros());
        }
    }
}
