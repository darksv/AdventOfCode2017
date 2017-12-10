fn main() {
    use std::io::BufRead;

    let file = std::fs::File::open("input.txt").unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap();
    let line = line.unwrap();
    let mut blocks: Vec<usize> = line
        .split('\t')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sequences: Vec<Vec<usize>> = vec![];
    loop {
        sequences.push(blocks.clone());

        // Find max value and index preserving first occurrence
        // max_by_key is not suitable here since it returns the last occurrence
        let (mut index, mut remaining) = {
            let mut index = 0;
            let mut value = blocks[0];
            for (i, &v) in blocks.iter().enumerate() {
                if v > value {
                    index = i;
                    value = v;
                }
            }
            (index, value)
        };

        blocks[index] = 0;
        while remaining > 0 {
            index = (index + 1) % blocks.len();
            blocks[index] += 1;
            remaining -= 1;
        }

        let previous_index = {
            let mut index = None;
            for (i, sequence) in sequences.iter().enumerate() {
                let is_equal = sequence
                    .iter()
                    .zip(&blocks)
                    .all(|(ref a, ref b)| a == b);

                if is_equal {
                    index = Some(i);
                    break;
                }
            }
            index
        };
        if let Some(index) = previous_index {
            println!("number of redistributions: {}, length: {}", sequences.len(), sequences.len() - index);
            break;
        }
    }
}