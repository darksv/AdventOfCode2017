#![feature(ascii_ctype)]
#![feature(entry_and_modify)]

#[derive(Clone, Debug)]
struct Node {
    name: String,
    ancestor: Option<usize>,
    children: Vec<usize>,
    weight: Option<usize>,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: vec![],
        }
    }

    fn insert(&mut self, name: &str) -> usize {
        self.nodes
            .iter()
            .position(|x| x.name == name)
            .unwrap_or_else(|| {
                let index = self.nodes.len();
                self.nodes.push(Node {
                    weight: None,
                    children: vec![],
                    ancestor: None,
                    name: name.to_owned(),
                });
                index
            })
    }

    fn find_root<'a>(&'a self) -> Option<(usize, &'a Node)> {
        self.nodes
            .iter()
            .enumerate()
            .find(|&(_, node)| node.ancestor.is_none())
    }
}

fn main() {
    use std::io::BufRead;

    let mut tree = Tree::new();

    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (node_name, node_weight, node_children) = parse_line(&line);
        let ancestor = tree.insert(node_name);

        let mut children = vec![];
        for node_name in node_children {
            let child = tree.insert(node_name);
            children.push(child);
            tree.nodes[child].ancestor = Some(ancestor);
        }

        let node = &mut tree.nodes[ancestor];
        node.weight = Some(node_weight);
        node.children = children;
    }

    if let Some((index, node)) = tree.find_root() {
        println!("1: {}", &node.name);
        println!("2: {}", find_invalid_weight(&tree.nodes, index).unwrap());
    }
}

fn find_invalid_weight(nodes: &Vec<Node>, root: usize) -> Option<usize> {
    match check_balance(nodes, root) {
        BalanceResult::NotBalanced { required_self_weight } => Some(required_self_weight),
        _ => None,
    }
}

#[derive(Debug)]
enum BalanceResult {
    Balanced {
        self_weight: usize,
        children_weight: usize,
    },
    NotBalanced {
        required_self_weight: usize,
    }
}

fn check_balance(nodes: &Vec<Node>, root: usize) -> BalanceResult {
    let node = nodes.get(root).unwrap();

    let mut sum_weight = 0;
    let mut weights = vec![];
    for child in &node.children {
        match check_balance(nodes, *child) {
            BalanceResult::Balanced { self_weight, children_weight } => {
                let weight = self_weight + children_weight;
                weights.push((weight, self_weight));
                sum_weight += weight;
            }
            result @ _ => return result
        }
    }

    let mut w = std::collections::HashMap::with_capacity(2);
    for &(weight, _self_weight) in &weights {
        w.entry(weight)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    if w.len() <= 1 {
        return BalanceResult::Balanced {
            self_weight: node.weight.unwrap_or(0),
            children_weight: sum_weight,
        };
    } else if w.len() > 2 {
        panic!("more than one node has different weight");
    }

    let actual_total_weight = w
        .iter()
        .find(|&x| x.1 == &1)
        .map(|x| x.0)
        .unwrap();

    let required_total_weight = w
        .keys()
        .find(|&x| x != actual_total_weight)
        .unwrap();

    let &(_, current_self_weight) = weights
        .iter()
        .find(|&&(weight, _)| weight == *actual_total_weight)
        .unwrap();

    let required_self_weight = if required_total_weight > actual_total_weight {
        current_self_weight + (required_total_weight - actual_total_weight)
    } else {
        current_self_weight - (actual_total_weight - required_total_weight)
    };

    BalanceResult::NotBalanced { required_self_weight }
}

fn parse_line<'a>(line: &'a str) -> (&'a str, usize, Vec<&'a str>) {
    let mut it = line.chars();
    let it = it.by_ref();

    let mut position = 0;

    let name = {
        let count = it
            .take_while(|&x| x != ' ')
            .count();
        position += count + 1;
        &line[..count]
    };

    // Skip left paren
    let _ = it.next();
    position += 1;
    let weight = {
        let offset = position;
        let count = it
            .take_while(|&x| char::is_digit(x, 10))
            .count();
        position += count;
        line[offset..offset + count].parse::<usize>().unwrap()
    };
    // Skip right paren
    let _ = it.next();
    position += 1;

    let rest = &line[position..];
    let children = if rest.is_empty() {
        vec![]
    } else {
        rest[4..].split(", ").collect()
    };

    (name, weight, children)
}
