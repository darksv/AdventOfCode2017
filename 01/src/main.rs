fn main() {
    use std::io::Read;

    let file = std::fs::File::open("input.txt").unwrap();
    let digits: Vec<_> = std::io::BufReader::new(file)
        .bytes()
        .map(|x| x.unwrap() - b'0')
        .collect();

    println!("1: {}", solve_captcha(&digits, 1));
    println!("2: {}", solve_captcha(&digits, digits.len() / 2));
}

fn solve_captcha(digits: &Vec<u8>, offset: usize) -> usize {
    let n = digits.len();
    let mut sum = 0usize;
    for i in 0..n {
        if digits[i] == digits[(i + offset) % n] {
            sum += digits[i] as usize;
        }
    }
    sum
}