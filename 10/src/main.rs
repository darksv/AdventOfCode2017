#![feature(io)]

fn main() {
    use std::io::Read;

    let content = {
        let mut content = String::new();
        let mut file = std::fs::File::open("input.txt").unwrap();
        file.read_to_string(&mut content).unwrap();
        content
    };

    let lengths: Vec<u8> = content
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", lengths);
}
