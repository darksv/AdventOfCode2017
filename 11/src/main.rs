#![feature(entry_and_modify)]

fn main() {
    let steps = read_steps();
    println!("1: {}", minimal_number_of_steps(&steps));
    println!("2: {}", longest_distance(&steps));
}

fn read_steps() -> Vec<Dir> {
    use std::io::Read;
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
        .split(',')
        .map(|part| {
            match part {
                "n" => Dir::N,
                "ne" => Dir::NE,
                "se" => Dir::SE,
                "s" => Dir::S,
                "sw" => Dir::SW,
                "nw" => Dir::NW,
                _ => panic!("invalid direction {}", part),
            }
        })
        .collect()
}

fn minimal_number_of_steps(steps: &[Dir]) -> usize {
    let mut counts = make_direction_counts();
    for &dir in steps {
        counts.entry(dir).and_modify(|x| *x += 1);
    }
    reduce_number_of_steps(&mut counts);
    counts.values().sum()
}

fn longest_distance(steps: &[Dir]) -> usize {
    let mut directions = make_direction_counts();
    steps
        .iter()
        .map(|&step| {
            directions.entry(step).and_modify(|x| *x += 1);
            reduce_number_of_steps(&mut directions);
            directions.values().sum()
        })
        .max()
        .unwrap_or(0)
}

fn make_direction_counts() -> std::collections::HashMap<Dir, usize> {
    let dirs = [
        Dir::N,
        Dir::S,
        Dir::SE,
        Dir::NE,
        Dir::SW,
        Dir::NW
    ];
    dirs
        .iter()
        .map(|&d| (d, 0))
        .collect()
}

fn reduce_number_of_steps(steps: &mut std::collections::HashMap<Dir, usize>) {
    cancel_opposite_steps(steps);
    simplify_steps(steps);
}

fn simplify_steps(steps: &mut std::collections::HashMap<Dir, usize>) {
    let possible_simplifications = [
        (Dir::NW, Dir::NE, Dir::N),
        (Dir::SW, Dir::SE, Dir::S),
        (Dir::S, Dir::NW, Dir::SW),
        (Dir::S, Dir::NE, Dir::SE),
        (Dir::N, Dir::SW, Dir::NW),
        (Dir::N, Dir::SE, Dir::NE),
    ];

    for &(first, second, result) in possible_simplifications.iter() {
        let first_count = steps[&first];
        let second_count = steps[&second];
        let result_count = steps[&result];
        let count = usize::min(first_count, second_count);
        if count > 0 {
            steps.insert(first, first_count - count);
            steps.insert(second, second_count - count);
            steps.insert(result, result_count + count);
        }
    }
}

fn cancel_opposite_steps(steps: &mut std::collections::HashMap<Dir, usize>) {
    let possible_cancels = [
        (Dir::N, Dir::S),
        (Dir::SE, Dir::NW),
        (Dir::NE, Dir::SW),
    ];

    for &(first, second) in possible_cancels.iter() {
        let first_count = steps[&first];
        let second_count = steps[&second];
        let count = usize::min(first_count, second_count);
        if count > 0 {
            steps.insert(first, first_count - count);
            steps.insert(second, second_count - count);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW
}