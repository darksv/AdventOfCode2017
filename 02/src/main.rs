#![feature(io)]

fn main() {
    use std::io::{Read, BufRead};

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut checksum = 0;
    for line in reader.lines() {
        let row: Vec<_> = line
            .unwrap()
            .split('\t')
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        for i in 0..row.len() {
            for j in i + 1..row.len() {
                let d1 = row[i];
                let d2 = row[j];

                let div = if d1 % d2 == 0 {
                    Some(d1 / d2)
                } else if d2 % d1 == 0 {
                    Some(d2 / d1)
                } else {
                    None
                };

                if let Some(d) = div {
                    checksum += d;
                }
            }
        }
//
//        let mut min = isize::max_value();
//        let mut max = isize::min_value();
//        for number in row {
//            if number < min {
//                min = number;
//            }
//            if number > max {
//                max = number;
//            }
//        }
//        checksum += max - min;
    }
    println!("checksum: {}", checksum);
}
