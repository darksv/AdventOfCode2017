const N: usize = 325489;

enum Dir {
    Right,
    Up,
    Left,
    Down,
}

fn main() {
    let (x, y) = SpiralGenerator::new().nth(N - 1).unwrap();
    println!("{}", isize::abs(x) + isize::abs(y));

    let mut items_by_position = std::collections::HashMap::new();
    for (x, y) in SpiralGenerator::new() {
        if x == 0 && y == 0 {
            items_by_position.insert((0, 0), 1);
            continue;
        }
        let neighbour_coords = [
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
            (x - 1, y), (x + 1, y),
            (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        ];
        let sum: usize = neighbour_coords
            .iter()
            .map(|x| items_by_position.get(x).unwrap_or(&0))
            .sum();

        if sum > N {
            println!("{}", sum);
            break;
        }

        items_by_position.insert((x, y), sum);
    }
}

struct SpiralGenerator {
    x_max: isize,
    x_min: isize,
    y_max: isize,
    y_min: isize,
    x: isize,
    y: isize,
    dir: Dir,
}

impl Iterator for SpiralGenerator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.y);

        let (x, y) = match self.dir {
            Dir::Right => {
                if self.x == self.x_max - 1 {
                    self.y_max += 1;
                    self.dir = Dir::Up;
                }
                (self.x + 1, self.y)
            }
            Dir::Up => {
                if self.y == self.y_max - 1 {
                    self.x_min -= 1;
                    self.dir = Dir::Left;
                }
                (self.x, self.y + 1)
            }
            Dir::Left => {
                if self.x == self.x_min + 1 {
                    self.y_min -= 1;
                    self.dir = Dir::Down;
                }
                (self.x - 1, self.y)
            }
            Dir::Down => {
                if self.y == self.y_min + 1 {
                    self.x_max += 1;
                    self.dir = Dir::Right;
                }
                (self.x, self.y - 1)
            }
        };
        self.x = x;
        self.y = y;
        Some(result)
    }
}

impl SpiralGenerator {
    pub fn new() -> Self {
        Self {
            x_max: 1,
            x_min: 0,
            y_max: 0,
            y_min: 0,
            x: 0,
            y: 0,
            dir: Dir::Right,
        }
    }
}
