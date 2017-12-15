const INPUT: &str = "jxqlasbh";

fn main() {
    println!("1: {}", count_used_squares(INPUT));
    println!("2: {}", count_groups_of_used(INPUT));
}

fn count_used_squares(input: &str) -> usize {
    let mut used_squares = 0;
    for i in 0..128 {
        let row = format!("{}-{}", input, i);
        used_squares += hash(row.as_bytes())
            .iter()
            .map(|&x| x.count_ones() as usize)
            .sum::<usize>();
    }
    used_squares
}

#[derive(Copy, Clone)]
enum Region {
    None,
    NotAssigned,
    Assigned(usize),
}

fn count_groups_of_used(input: &str) -> usize {
    let mut grid = build_grid(input);
    let mut to_visit = std::collections::VecDeque::<(usize, usize)>::new();
    let mut groups = 0;
    while let Some((s, t)) = first_not_assigned(&grid) {
        to_visit.push_front((s, t));
        while let Some((i, j)) = to_visit.pop_back() {
            let region = &mut grid[i][j];
            match *region {
                Region::NotAssigned => {
                    *region = Region::Assigned(groups);

                    if i > 0 {
                        to_visit.push_front((i - 1, j));
                    }

                    if i < 127 {
                        to_visit.push_front((i + 1, j));
                    }

                    if j > 0 {
                        to_visit.push_front((i, j - 1));
                    }

                    if j < 127 {
                        to_visit.push_front((i, j + 1));
                    }
                }
                _ => continue,
            }
        }
        groups += 1;
    }
    groups
}

fn build_grid(input: &str) -> [[Region; 128]; 128] {
    let mut regions = [[Region::None; 128]; 128];
    for i in 0..128 {
        let row = format!("{}-{}", input, i);
        for (byte_idx, &byte) in hash(row.as_bytes()).iter().enumerate() {
            let mut byte = byte;
            for bit_idx in (0..8).rev() {
                let j = byte_idx * 8 + bit_idx;
                if (byte & 1) == 1 {
                    regions[i][j] = Region::NotAssigned
                }
                byte >>= 1;
            }
        }
    }
    regions
}

fn first_not_assigned(grid: &[[Region; 128]; 128]) -> Option<(usize, usize)> {
    for i in 0..128 {
        for j in 0..128 {
            if let Region::NotAssigned = grid[i][j] {
                return Some((i, j));
            }
        }
    }
    None
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