fn main() {
    let line = {
        use std::io::Read;

        let mut file = std::fs::File::open("input.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };

    let mut blocks: Vec<usize> = line
        .split('\t')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sequences: Vec<Vec<usize>> = vec![];
    loop {
        sequences.push(blocks.clone());
        let index_of_max = first_index_of_max(&blocks).unwrap();
        redistribute(&mut blocks, index_of_max);
        let previous_index = sequences
            .iter()
            .position(|sequence| {
                are_slices_equal(&blocks, &sequence)
            });
        if let Some(index) = previous_index {
            println!("1: {}", sequences.len());
            println!("2: {}", sequences.len() - index);
            break;
        }
    }
}

fn redistribute(blocks: &mut Vec<usize>, from: usize) {
    let mut index = from;
    let mut remaining = std::mem::replace(&mut blocks[index], 0);
    while remaining > 0 {
        index = (index + 1) % blocks.len();
        blocks[index] += 1;
        remaining -= 1;
    }
}

fn first_index_of_max<T: PartialOrd + Copy>(numbers: &Vec<T>) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }
    let mut max_index = 0;
    let mut max_value = numbers[0];
    for (index, &value) in numbers.iter().enumerate() {
        if value > max_value {
            max_index = index;
            max_value = value;
        }
    }
    Some(max_index)
}

fn are_slices_equal<T: PartialEq>(first: &[T], seconds: &[T]) -> bool {
    if first.len() != seconds.len() {
        return false;
    }
    first
        .iter()
        .zip(seconds.iter())
        .all(|(ref a, ref b)| a == b)
}