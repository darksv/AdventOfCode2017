#![feature(io)]

fn main() {
    let mut garbage = false;
    let mut level = 0;
    let mut skip = 0;
    let mut garbage_count = 0;
    let mut score = 0;

    use std::io::Read;
    let file = std::fs::File::open("input.txt").unwrap();
    for c in file.chars() {
        let c = c.unwrap();
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            '{' if !garbage => {
                level += 1;
                score += level;
            }
            '}' if !garbage => {
                level -= 1;
            }
            '<' if !garbage => {
                garbage = true;
            }
            '>' if garbage => {
                garbage = false;
            }
            '!' if garbage => {
                skip = 1;
            }
            _ if garbage => {
                garbage_count += 1;
            }
            _ => (),
        }
    }

    println!("1: {}", score);
    println!("2: {}", garbage_count);
}
