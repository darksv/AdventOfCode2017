fn main() {
    let layers = read_layers();
    println!("1: {}", calculate_severity(&layers));
    println!("2: {}", calculate_min_delay(&layers));
}

fn read_layers() -> Vec<usize> {
    use std::io::BufRead;

    let mut layers = vec![];

    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut layer = None;
        let mut range = None;
        for (i, n) in line.split(": ").enumerate() {
            match i {
                0 => layer = Some(n.parse::<usize>().unwrap()),
                1 => range = Some(n.parse::<usize>().unwrap()),
                _ => panic!()
            }
        }

        let layer = layer.unwrap();
        let range = range.unwrap();

        while layers.len() < layer + 1 {
            layers.push(0);
        }
        layers[layer] = range;
    }
    layers
}

fn calculate_severity(layers: &[usize]) -> usize {
    layers
        .iter()
        .enumerate()
        .filter(|&(i, &range)| position_at(i, range) == Some(0))
        .map(|(i, &range)| i * range)
        .sum()
}

fn calculate_min_delay(layers: &[usize]) -> usize {
    (0usize..)
        .take_while(|&delay| is_caught(&layers, delay))
        .count()
}

fn is_caught(layers: &[usize], delay: usize) -> bool {
    layers
        .iter()
        .enumerate()
        .any(|(i, &range)| position_at(delay + i, range) == Some(0))
}

fn position_at(steps: usize, range: usize) -> Option<usize> {
    if range == 0 {
        return None;
    }
    let cycle = if range == 1 { 1 } else { (range - 1) * 2 };
    let phase = steps % cycle;
    let position = if phase < range { phase } else { cycle - phase };
    Some(position)
}