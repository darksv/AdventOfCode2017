fn main() {
    let components = read_components();
    println!("1: {}", calculate_max_strength(0, components));
}

fn calculate_max_strength(end: usize, components: Vec<Component>) -> usize {
    let mut strength = 0;
    for component in &components {
        if let Some(other_end) = component.get_other_end(end) {
            let filtered = components
                .iter()
                .filter(|&x| *x != *component)
                .map(|x| *x)
                .collect();

            let current_strength = end + other_end +
                calculate_max_strength(other_end, filtered);
            strength = strength.max(current_strength);
        }
    }
    return strength;
}

fn read_components() -> Vec<Component> {
    use std::io::BufRead;

    let mut components = vec![];
    let file = std::fs::File::open("input.txt").expect("some input");
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        components.push(parse_component(&line));
    }
    components
}

fn parse_component(s: &str) -> Component {
    let mut i = s.split('/').flat_map(|x| x.parse::<usize>().ok());
    Component(
        i.next().unwrap(),
        i.next().unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Component(usize, usize);

impl Component {
    pub fn get_other_end(&self, end: usize) -> Option<usize> {
        if self.0 == end {
            Some(self.1)
        } else if self.1 == end {
            Some(self.0)
        } else {
            None
        }
    }
}