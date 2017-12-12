use std::collections::HashMap;
use std::collections::HashSet;

type Graph = HashMap<usize, Vec<usize>>;

fn main() {
    let nodes = read_nodes();
    println!("1: {}", count_nodes_in_group_with(&nodes, 0));
    println!("2: {}", count_groups(&nodes));
}

fn read_nodes() -> Graph {
    use std::io::BufRead;
    let mut nodes: Graph = Graph::default();
    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let index_of_space = line.chars().position(|x| x == ' ').unwrap();
        let node_id: usize = line[..index_of_space].parse().unwrap();

        let node_ids: Vec<_> = line[index_of_space + 5..]
            .split(", ")
            .map(|c| c.parse().unwrap())
            .collect();

        nodes.insert(node_id, node_ids);
    }
    nodes
}

fn count_nodes_in_group_with(nodes: &Graph, node: usize) -> usize {
    let mut visited = HashSet::new();
    perform_dfs(&nodes, &mut visited, node);
    visited.len()
}

fn count_groups(nodes: &Graph) -> usize {
    let mut visited = HashSet::new();
    // Execute BFS algorithm until there are nodes to visit starting from 0th node.
    let mut number_of_groups = 0;
    while let Some(not_visited) = any_not_visited(&nodes, &visited) {
        perform_dfs(&nodes, &mut visited, not_visited);
        number_of_groups += 1;
    }
    number_of_groups
}

fn perform_dfs(nodes: &Graph, visited: &mut HashSet<usize>, from: usize) {
    let mut to_visit = vec![from];
    while let Some(node_id) = to_visit.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id);
        for &connected_node in &nodes[&node_id] {
            to_visit.push(connected_node);
        }
    }
}

fn any_not_visited(nodes: &Graph, visited: &HashSet<usize>) -> Option<usize> {
    nodes
        .keys()
        .find(|node_id| !visited.contains(node_id))
        .map(|&x| x)
}