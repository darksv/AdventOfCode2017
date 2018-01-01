#![feature(entry_and_modify)]

extern crate num;

fn main() {
    let mut particles = read_input()
        .expect("some good input");

    let (index_of_closest, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| p.a.euclidean_length_squared())
        .expect("at least one particle?");
    println!("1: {}", index_of_closest);

    let mut last_count = 0;
    loop {
        tick(&mut particles);
        let current_count = particles.iter().filter(|p| !p.destroyed).count();
        if last_count != current_count {
            println!("2: {} (take last when the value is not changing... will be automated soon...)", current_count);
        }
        last_count = current_count;
    }
}

use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
struct Vector3D<T: num::Num + num::Signed + Copy>(T, T, T);

impl<T> Add for Vector3D<T> where T: num::Num + num::Signed + Copy {
    type Output = Vector3D<T>;
    fn add(self, rhs: Vector3D<T>) -> Self::Output {
        Vector3D::<T>(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl<T> AddAssign for Vector3D<T> where T: num::Num + num::Signed + Copy {
    fn add_assign(&mut self, rhs: Vector3D<T>) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self.2 = self.2 + rhs.2;
    }
}

trait VectorLength {
    type Output;
    fn manhattan_length(&self) -> Self::Output;
    fn euclidean_length_squared(&self) -> Self::Output;
}

impl<T> VectorLength for Vector3D<T> where T: num::Num + num::Signed + Copy {
    type Output = T;
    fn manhattan_length(&self) -> Self::Output {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
    fn euclidean_length_squared(&self) -> Self::Output {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

fn tick(particles: &mut Vec<Particle>) {
    use std::collections::HashMap;
    use std::collections::hash_map::Entry;

    let mut current_positions: HashMap<_, Vec<usize>> = HashMap::new();

    for (idx, particle) in particles.iter_mut().enumerate() {
        if particle.destroyed {
            continue;
        }

        particle.v += particle.a;
        particle.p += particle.v;

        match current_positions.entry(particle.p) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(idx);
            },
            Entry::Vacant(mut entry) => {
                entry.insert(vec![idx]);
            }
        }
    }

    current_positions
        .values()
        .into_iter()
        .filter(|x| x.len() > 1)
        .flat_map(|x| x)
        .for_each(|&id| {
            particles[id].destroyed = true;
        });
}

#[derive(Debug)]
struct Particle {
    p: Vector3D<isize>,
    v: Vector3D<isize>,
    a: Vector3D<isize>,
    destroyed: bool,
}

fn read_input() -> Option<Vec<Particle>> {
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open("input.txt").ok()?;
    let particles = BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok())
        .flat_map(|line| parse_particle(&line.unwrap()))
        .collect();
    Some(particles)
}

fn parse_particle(line: &str) -> Option<Particle> {
    let mut i = line.split(", ");
    Some(Particle {
        p: parse_item(&i.next()?)?,
        v: parse_item(&i.next()?)?,
        a: parse_item(&i.next()?)?,
        destroyed: false,
    })
}

fn parse_item(s: &str) -> Option<Vector3D<isize>> {
    let end = s.len() - 1;
    parse_vector3d(&s[3..end])
}

fn parse_vector3d(s: &str) -> Option<Vector3D<isize>> {
    let mut i = s.split(',');
    Some(Vector3D(
        i.next()?.parse().ok()?,
        i.next()?.parse().ok()?,
        i.next()?.parse().ok()?,
    ))
}