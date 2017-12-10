fn main() {
    use std::io::BufRead;

    let mut valid1 = 0;
    let mut valid2 = 0;

    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if is_valid_passphrase1(&line) {
            valid1 += 1;
        }
        if is_valid_passphrase2(&line) {
            valid2 += 1;
        }
    }

    println!("1: {}", valid1);
    println!("2: {}", valid2);
}

fn is_valid_passphrase1(s: &str) -> bool {
    let mut unique_words = std::collections::HashSet::new();
    for word in s.split(' ') {
        if unique_words.contains(&word) {
            return false;
        } else {
            unique_words.insert(word);
        }
    }
    true
}

fn is_valid_passphrase2(s: &str) -> bool {
    let mut unique_words = std::collections::HashSet::new();
    for word in s.split(' ') {
        let word = sort_letters(word);
        if unique_words.contains(&word) {
            return false;
        } else {
            unique_words.insert(word);
        }
    }
    true
}

fn sort_letters(s: &str) -> String {
    use std::iter::FromIterator;

    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    String::from_iter(chars.iter())
}