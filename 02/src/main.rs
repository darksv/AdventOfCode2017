fn main() {
    use std::io::{BufRead};

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut checksum1 = 0;
    let mut checksum2 = 0;
    for line in reader.lines() {
        let numbers: Vec<isize> = line
            .unwrap()
            .split('\t')
            .map(|x| x.parse().unwrap())
            .collect();

        if let Some(checksum) = checksum_minmax(&numbers) {
            checksum1 += checksum;
        }

        if let Some(checksum) = checksum_div(&numbers) {
            checksum2 += checksum;
        }
    }
    println!("1: {}", checksum1);
    println!("2: {}", checksum2);
}

fn checksum_minmax(numbers: &Vec<isize>) -> Option<isize> {
    min_max(&numbers).map(|(min, max)| max - min)
}

fn min_max(numbers: &Vec<isize>) -> Option<(isize, isize)> {
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
    Some((min, max))
}

fn checksum_div(numbers: &Vec<isize>) -> Option<isize> {
    find_divisible_numbers(&numbers).map(|(greater, lesser)| greater / lesser)
}

fn find_divisible_numbers(numbers: &Vec<isize>) -> Option<(isize, isize)> {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            if a % b == 0 {
                return Some((a, b))
            } else if b % a == 0 {
                return Some((b, a))
            };
        }
    }
    None
}