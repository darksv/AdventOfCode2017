fn main() {
    use std::io::{BufRead};

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut checksums = (0, 0);
    for line in reader.lines() {
        let row: Vec<isize> = line
            .unwrap()
            .split('\t')
            .map(|x| x.parse().unwrap())
            .collect();

        if let (Some(c1), Some(c2)) = (checksum1(&row), checksum2(&row)) {
            checksums = (checksums.0 + c1, checksums.1 + c2);
        }

    }
    println!("1: {}", checksums.0);
    println!("2: {}", checksums.1);
}

fn checksum1(numbers: &Vec<isize>) -> Option<isize> {
    if numbers.is_empty() {
        return None;
    }
    let mut min = numbers[0];
    let mut max = numbers[0];
    for number in &numbers[1..] {
        let number = *number;
        if number < min {
            min = number;
        }
        if number > max {
            max = number;
        }
    }
    Some(max - min)
}

fn checksum2(numbers: &Vec<isize>) -> Option<isize> {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            if a % b == 0 {
                return Some(a / b)
            } else if b % a == 0 {
                return Some(b / a)
            };
        }
    }
    None
}