fn main() {
    use std::io::Read;

    let file = std::fs::File::open("input.txt").unwrap();
    let digits: Vec<_> = std::io::BufReader::new(file)
        .bytes()
        .map(|x| x.unwrap() - b'0')
        .collect();

    let n = digits.len();
    let offset = n / 2;
    let mut sum = 0usize;
    for i in 0..n {
        if digits[i] == digits[(i + offset) % n] {
            sum += digits[i] as usize;
        }
    }

    println!("{}", sum);
}
