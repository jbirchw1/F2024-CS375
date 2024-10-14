use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn print_graph(graph: &HashMap<i32, Vec<i32>>) {
    println!("adjacency list for each edge:");
    for (node, neighbors) in graph {
        println!("{}: {:?}", node, neighbors);
    }
}

pub fn build_graph_from_stdin() -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let stdin = io::stdin();

    // Read each line of input
    for line in stdin.lock().lines().skip(1) {
        let input = line.unwrap();

        // Parse the edge
        let mut edge = input.split_whitespace();
        let u: i32 = edge.next().unwrap().parse().unwrap();
        let v: i32 = edge.next().unwrap().parse().unwrap();

        // Insert the directed edge from u to v (u -> v)
        graph.entry(u).or_insert_with(Vec::new).push(v);
    }

    graph // Return ownership of the graph
}
