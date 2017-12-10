#![feature(ascii_ctype)]

#[derive(Clone, Debug)]
struct Node {
    name: String,
    ancestor: Option<usize>,
    children: Vec<usize>,
    weight: Option<usize>,
}

fn main() {
    use std::io::BufRead;

    let mut nodes = vec![];

    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (node_name, node_weight, node_children) = parse_line(&line);
        let ancestor = insert_node(&mut nodes, node_name);

        let mut children = vec![];
        for node_name in node_children {
            let child = insert_node(&mut nodes, node_name);
            children.push(child);
            nodes[child].ancestor = Some(ancestor);
        }

        let node = &mut nodes[ancestor];
        node.weight = Some(node_weight);
        node.children = children;
    }

    let (index, node) = nodes
        .iter()
        .enumerate()
        .find(|&(_, node)| node.ancestor.is_none())
        .unwrap();

    println!("root: {:?} {:?}", &node, get_fixed_weight(&nodes, index));
}

fn count_weight(nodes: &Vec<Node>, root: usize) -> (usize, usize) {
    let node = nodes.get(root).unwrap();

    let mut sum_weight = 0;
    let mut weights = vec![];
    for child in &node.children {
        let (self_weight, children_weight) = count_weight(nodes, *child);
        let weight = self_weight + children_weight;
        weights.push((weight, self_weight));
        sum_weight += weight;
    }

    if !weights.iter().zip(weights.iter().skip(1)).all(|(a, b)| a.0 == b.0) {
        println!("{} {} {} {:?}", node.weight.unwrap_or(0) + sum_weight, node.weight.unwrap_or(0), sum_weight, weights);
    }

    (node.weight.unwrap_or(0), sum_weight)
}

fn get_fixed_weight(nodes: &Vec<Node>, root: usize) -> Option<usize> {
    // BFS algorithm, don't have to care about breaking cycles since won't have one...
//    let mut current_anc = root;

    let mut indents = std::collections::HashMap::new();
    indents.insert(root, 0);

    let mut queue = std::collections::VecDeque::new();
    queue.push_front(root);

//    let mut a = None;
//
//    let mut m = vec![];

    while let Some(index) = queue.pop_back() {
        let indent: usize = *indents.get(&index).unwrap();
        let current = &nodes[index];

//        if a == Some(index) {
//            m.push(index);
//        }else {
//            a = Some(index);
//            m.clear();
//            m.push(index);
//        }


        for _ in 0..indent {
            print!(" ");
        }

        println!("{} ({})", current.name, current.weight.unwrap_or(0));
        for c in &current.children {
            indents.insert(*c, indent + 1);



            queue.push_front(*c);
        }

//        println!("{:?}", m);
    }

    Some(root)
}


fn insert_node(nodes: &mut Vec<Node>, name: &str) -> usize {
    nodes
        .iter()
        .position(|x| x.name == name)
        .unwrap_or_else(|| {
            let index = nodes.len();
            nodes.push(Node {
                weight: None,
                children: vec![],
                ancestor: None,
                name: name.to_owned(),
            });
            index
        })
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
