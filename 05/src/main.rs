fn main() {
    use std::io::BufRead;

    let file = std::fs::File::open("input.txt").unwrap();
    let jumps: Vec<isize> = std::io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let count1 = execute(jumps.clone(), |_offset, j| {
        *j += 1;
    });
    println!("1: {}", count1);

    let count2 = execute(jumps, |offset, j| {
        if offset >= 3 {
            *j -= 1;
        } else {
            *j += 1;
        }
    });
    println!("2: {}", count2);
}

fn execute<F>(mut jumps: Vec<isize>, modifier: F) -> usize
    where F: Fn(isize, &mut isize)
{
    let mut count = 0;
    let mut pc = 0;
    while pc < jumps.len() {
        count += 1;
        let offset = jumps[pc];
        modifier(offset, &mut jumps[pc]);
        pc = if offset < 0 {
            pc - (-offset) as usize
        } else {
            pc + offset as usize
        };
    }
    count
}