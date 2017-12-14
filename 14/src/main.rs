const INPUT: &str = "jxqlasbh";

fn main() {
    println!("1: {}", count_used_squares());
}

fn count_used_squares() -> usize {
    let mut used_squares = 0;
    for i in 0..128 {
        let row = format!("{}-{}", INPUT, i);
        used_squares += hash(row.as_bytes())
            .iter()
            .map(|&x| x.count_ones() as usize)
            .sum::<usize>();
    }
    used_squares
}


fn hash(bytes: &[u8]) -> [u8; 16] {
    let mut buffer: Vec<u8> = (0..256).map(|x| x as u8).collect();
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