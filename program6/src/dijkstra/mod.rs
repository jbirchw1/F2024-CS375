use std::collections::{HashMap};
use std::io::{self, BufRead};

pub fn print_graph(graph: &HashMap<i32, Vec<(i32, i32)>>) {
    println!("adjacency list for each edge:");
    for (node, neighbors) in graph {
        println!("{}: {:?}", node, neighbors);
    }
}

pub fn build_graph_from_stdin() -> HashMap<i32, Vec<(i32, i32)>> {
    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let stdin = io::stdin();

    // Read each line of input
    for line in stdin.lock().lines().skip(1) {
        let input = line.unwrap();

        // Parse the edge
        let mut edge = input.split_whitespace();
        let u: i32 = edge.next().unwrap().parse().unwrap();
        let v: i32 = edge.next().unwrap().parse().unwrap();
        let dist: i32 = edge.next().unwrap().parse().unwrap();

        // if u exists in the hash map, insert v into its' vector
        // if not, create u and then add v
        graph.entry(u).or_insert_with(Vec::new).push((v, dist));
    }

    graph // Return ownership of the graph
}

pub fn shortest_path() {
    println!("TODO: shortest path");
}