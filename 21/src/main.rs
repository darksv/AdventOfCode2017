#![feature(iterator_step_by)]
#![feature(slice_rotate)]

mod patterns;
use patterns::Pattern;

type Rule = (Pattern, Pattern);

fn main() {
    let rules = read_rules();
    let pattern = Pattern::from_str(".#./..#/###");

    println!("1: {}", get_transformed(pattern.clone(), &rules, 5).count_set_pixels());
    println!("2: {}", get_transformed(pattern.clone(), &rules, 18).count_set_pixels());
}

fn get_transformed(pattern: Pattern, rules: &[Rule], iterations: usize) -> Pattern {
    let mut pattern = pattern;
    for _ in 0..iterations {
        let mut subpatterns = pattern.split();
        for mut subpattern in subpatterns.iter_mut() {
            *subpattern = find_replacement_for(subpattern, &rules)
                .expect("some valid rule, maybe?")
                .clone();
        }
        pattern = Pattern::merge(&subpatterns);
    }
    pattern
}

fn find_replacement_for<'a>(pattern_to_replace: &Pattern, rules: &'a [Rule]) -> Option<&'a Pattern> {
    for &(ref pattern, ref replacement) in rules {
        if pattern.is_variation_of(pattern_to_replace) {
            return Some(replacement);
        }
    }
    None
}

fn read_rules() -> Vec<Rule> {
    let mut rules = vec![];
    let file = std::fs::File::open("input.txt").unwrap();
    use std::io::BufRead;
    for line in std::io::BufReader::new(file).lines() {
        let rule = line.unwrap();
        let rule = parse_rule(&rule);
        rules.push(rule);
    }
    rules
}

fn parse_rule(rule: &str) -> Rule {
    let mut i = rule.split(" => ").map(|x| Pattern::from_str(x));
    (i.next().unwrap(), i.next().unwrap())
}
