fn main() {
    use std::io::BufRead;

    let file = std::fs::File::open("input.txt").unwrap();
    let jumps: Vec<isize> = std::io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    println!("jumps: {}", execute(jumps));
}

fn execute(mut j: Vec<isize>) -> usize {
    let mut jumps = 0;
    let mut program_counter = 0;
    while program_counter < j.len() {
        jumps += 1;
        let offset = j[program_counter];
        if offset >= 3 {
            j[program_counter] -= 1;
        } else {
            j[program_counter] += 1;
        }
        program_counter = if offset < 0 {
            program_counter - (-offset) as usize
        } else {
            program_counter + offset as usize
        };
    }
    jumps
}