fn main() {
    println!("1: {}", do_first());
    println!("2: {}", do_second());
}

fn do_first() -> usize {
    let mut infected: std::collections::HashSet<(isize, isize)> = read_infected().into_iter().collect();
    let mut cleaned = std::collections::HashSet::new();
    let mut infections = 0;
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;
    for _ in 0..10000 {
        let pos = (x, y);
        if infected.contains(&pos) {
            dir = dir.turn_right();
            infected.remove(&pos);
            cleaned.insert(pos);
        } else {
            dir = dir.turn_left();
            infected.insert(pos);
            infections += 1;
        }

        match dir {
            Dir::Up => y += 1,
            Dir::Down => y -= 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
        }
    }
    infections
}

fn do_second() -> usize {
    use std::collections::hash_map::HashMap;

    let mut nodes = HashMap::new();
    for node in read_infected().into_iter() {
        nodes.insert(node, NodeStatus::Infected);
    }

    let mut infections = 0;
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;
    for _ in 0..10000000 {
        let pos = (x, y);
        let status = nodes
            .entry(pos)
            .or_insert(NodeStatus::Clean);

        match *status {
            NodeStatus::Infected => {
                dir = dir.turn_right();
                *status = NodeStatus::Flagged;
            }
            NodeStatus::Weakened => {
                *status = NodeStatus::Infected;
                infections += 1;
            }
            NodeStatus::Clean => {
                dir = dir.turn_left();
                *status = NodeStatus::Weakened;
            }
            NodeStatus::Flagged => {
                dir = dir.reverse();
                *status = NodeStatus::Clean;
            }
        }

        match dir {
            Dir::Up => y += 1,
            Dir::Down => y -= 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
        }
    }
    infections
}

enum NodeStatus {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_left(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    fn turn_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn reverse(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn read_infected() -> Vec<(isize, isize)> {
    let mut infected = vec![];

    let file = std::fs::File::open("input.txt").unwrap();
    use std::io::BufRead;

    let mut y = 12;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut x = -12;
        for c in line.chars() {
            if c == '#' {
                infected.push((x, y));
            }
            x += 1;
        }
        y -= 1;
    }
    infected
}
