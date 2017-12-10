fn main() {
    use std::io::BufRead;
    let file = std::fs::File::open("input.txt").unwrap();

    let valid = std::io::BufReader::new(file)
        .lines()
        .filter(|ref x| is_valid_passphrase2(x.as_ref().unwrap()))
        .count();

    println!("valid passphrases: {}", valid);
}

fn is_valid_passphrase(s: &str) -> bool {
    let mut unique_words = std::collections::HashSet::new();
    for word in s.split(" ") {
        if unique_words.contains(word) {
            return false;
        } else {
            unique_words.insert(word);
        }
    }
    true
}

fn is_valid_passphrase2(s: &str) -> bool {
    let mut unique_words = std::collections::HashSet::new();
    for word in s.split(" ") {
        let word = standardize(&word);
        if unique_words.contains(&word) {
            return false;
        } else {
            unique_words.insert(word);
        }
    }
    true
}

fn standardize(s: &str) -> String {
    use std::iter::FromIterator;

    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    String::from_iter(chars.iter())
}