fn main() {
    println!("1: {}", count_all());
    println!("2: {}", count_divisible());
}

fn count_all() -> usize {
    let g1 = Generator::new(16807, 679);
    let g2 = Generator::new(48271, 771);

    count_compatible(40_000_000, g1, g2)
}

fn count_divisible() -> usize {
    let g1 = Generator::new(16807, 679)
        .filter(|&x| x % 4 == 0);
    let g2 = Generator::new(48271, 771)
        .filter(|&x| x % 8 == 0);

    count_compatible(5_000_000, g1, g2)
}

fn count_compatible<G1, G2>(n: usize, g1: G1, g2: G2) -> usize
    where
        G1: Iterator<Item=usize>,
        G2: Iterator<Item=usize>,
{
    g1
        .zip(g2)
        .take(n)
        .filter(|&(a, b)| (a as u16) == (b as u16))
        .count()
}

struct Generator {
    divider: usize,
    factor: usize,
    current: usize,
}

impl Generator {
    fn new(factor: usize, start: usize) -> Self {
        Generator {
            divider: 2147483647,
            factor,
            current: start,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.current * self.factor) % self.divider;
        self.current = current;
        Some(current)
    }
}