fn main() {
    println!("1: {}", find_number_after_last_inserted(2017, 380).unwrap());
    println!("2: {}", find_number_after_zero(50_000_000, 380).unwrap());
}

fn find_number_after_last_inserted(iterations: usize, skip: usize) -> Option<usize> {
    let mut position = 0;
    let mut buffer = vec![0usize];
    for i in 1..(iterations + 1) {
        position = (position + skip) % buffer.len() + 1;
        buffer.insert(position, i);
    }
    buffer.get(position + 1).map(|x| *x)
}

fn find_number_after_zero(iterations: usize, skip: usize) -> Option<usize> {
    let mut position = 0;
    let mut length = 1;
    let mut item = None;
    for i in 1..(iterations + 1) {
        position = (position + skip) % length + 1;
        length += 1;
        if position == 1 {
            item = Some(i);
        }
    }
    item
}
