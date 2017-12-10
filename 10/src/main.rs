#![feature(io)]

fn main() {
    use std::io::Read;

    let content = {
        let mut content = String::new();
        let mut file = std::fs::File::open("input.txt").unwrap();
        file.read_to_string(&mut content).unwrap();
        content
    };

    println!("1: {}", hash1(&content));
    println!("2: {}", hexify_bytes(&hash2(content.as_bytes())));
}

fn hash1(s: &str) -> usize {
    let lengths = s
        .split(',')
        .map(|x| x.parse::<u8>().unwrap());

    let mut list: Vec<usize> = (0..256).collect();
    let mut current = 0;
    let mut skip = 0;

    for length in lengths {
        reverse_circular(&mut list, current, length as usize);
        current += length as usize + skip;
        skip += 1;
    }

    (list[0] as usize) * (list[1] as usize)
}

fn hash2(bytes: &[u8]) -> [u8; 16] {
    let mut buffer: Vec<u8> = (0..256)
        .map(|x| x as u8)
        .collect();
    let mut current = 0;
    let mut skip = 0;

    let suffix = &[17u8, 31, 73, 47, 23];
    for _ in 0..64 {
        for &byte in bytes.iter().chain(suffix.iter()) {
            reverse_circular(&mut buffer, current, byte as usize);
            current += byte as usize + skip;
            skip += 1;
        }
    }

    let mut hash = [0u8; 16];
    for (i, byte) in buffer.iter().enumerate() {
        hash[i / 16] ^= byte;
    }
    hash
}

fn hexify_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write;

    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut hex, "{:02x}", byte).unwrap();
    }
    hex
}


fn reverse_circular<T: Sized>(buffer: &mut [T], from: usize, length: usize) {
    let mut lower = from;
    let mut upper = from + length - 1;
    let n = buffer.len();
    while lower < upper {
        buffer.swap(lower % n, upper % n);
        lower += 1;
        upper -= 1;
    }
}