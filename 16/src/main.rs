#![feature(slice_rotate)]

#[derive(Debug, Clone, Copy)]
enum Move<T: Sized> {
    Spin(usize),
    Exchange(usize, usize),
    Partner(T, T),
}

fn main() {
    use std::iter::FromIterator;
    let mut buffer = [
        'a', 'b', 'c', 'd',
        'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p'
    ];

    let moves = read_moves();
    perform_moves(&mut buffer, &moves);
    println!("1: {}", String::from_iter(&buffer));

    for _ in 1..(1_000_000_000 % find_cycle(buffer, &moves)) {
        perform_moves(&mut buffer, &moves);
    }

    println!("2: {}", String::from_iter(&buffer));
}

fn find_cycle(initial: [char; 16], moves: &[Move<char>]) -> usize {
    let mut history: Vec<[char; 16]> = vec![];
    let mut buffer = initial;
    loop {
        history.push(buffer.clone());
        perform_moves(&mut buffer, &moves);
        let was_previously = history
            .iter()
            .any(|sequence|
                sequence
                    .iter()
                    .zip(buffer.iter())
                    .all(|(a, b)| a == b)
            );
        if was_previously {
            return history.len();
        }
    }
}

fn perform_moves<T: Sized + PartialEq + Copy>(items: &mut [T], moves: &[Move<T>]) {
    for &mov in moves {
        match mov {
            Move::Spin(count) => {
                let length = items.len();
                items.rotate(length - count as usize)
            }
            Move::Exchange(a, b) => {
                items.swap(a as usize, b as usize)
            }
            Move::Partner(a, b) => {
                let (a, b) = indices_by_items(items, a, b);
                items.swap(a, b);
            }
        }
    }
}

fn indices_by_items<T: Sized + PartialEq + Copy>(d: &[T], a: T, b: T) -> (usize, usize) {
    let mut idx_a = None;
    let mut idx_b = None;
    for (idx, &item) in d.iter().enumerate() {
        if item == a {
            idx_a = Some(idx);
            if idx_b.is_some() {
                break;
            }
        }
        if item == b {
            idx_b = Some(idx);
            if idx_a.is_some() {
                break;
            }
        }
    }
    (idx_a.unwrap(), idx_b.unwrap())
}

fn read_moves() -> Vec<Move<char>> {
    use std::io::Read;
    let mut content = String::new();
    std::fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut moves = vec![];
    for part in content.split(',') {
        let mut it = part.chars();
        let it = it.by_ref();
        let op = it.next().unwrap();
        let mov = match op {
            's' => {
                let a = part[1..].parse::<usize>().unwrap();
                Move::Spin(a)
            }
            'x' => {
                let mut it = part[1..].split('/');
                let it = it.by_ref();

                let a = it.next().unwrap().parse().unwrap();
                let b = it.next().unwrap().parse().unwrap();

                Move::Exchange(a, b)
            }
            'p' => {
                let a = it.next().unwrap();
                let _ = it.next().unwrap();
                let b = it.next().unwrap();

                Move::Partner(a, b)
            }
            _ => panic!()
        };
        moves.push(mov);
    }

    moves
}
