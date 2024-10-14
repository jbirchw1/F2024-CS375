use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

/**
 * Small debug function to print each vertex and adjacency list
 * Takes graph (hashmap) as input
 */
pub fn print_graph(graph: &HashMap<i32, Vec<i32>>) {
    println!("adjacency list for each edge:");
    for (node, neighbors) in graph {
        println!("{}: {:?}", node, neighbors);
    }
}

/**
 * Parse the input and create the hashmap representation of the graph
 * creates a hashmap with each entry corresponding to a vertex, each
 * emtry associated with an adjacency list vector
 * returns the hasmap representation of the graph with i32 keys
 */
// TODO: enhance graph to be able to handle type <T> keys 
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

        // if u exists in the hash map, insert v into its' vector
        // if not, create u and then add v
        graph.entry(u).or_insert_with(Vec::new).push(v);
    }

    graph // Return ownership of the graph
}

/**
 * Breadth first search algorithm
 * Visits each vertex n distance away from the root until destination
 * is found or frontier queue is empty
 * Takes in the adjacency list graph as input and returns an option of distance
 */
pub fn breadth_first_search(
    start: i32,
    destination: i32,
    graph: &HashMap<i32, Vec<i32>>,
) -> Option<i32> {
    let mut visited = HashMap::new(); // visited nodes
    let mut distance = HashMap::new(); // distances from start node
    let mut queue = VecDeque::new(); // frontier queue

    // mark start as visited, set its distance to 0, and enqueue it
    visited.insert(start, true);
    distance.insert(start, 0);
    queue.push_back(start);

    // while the frontier queue is not empty
    while let Some(node) = queue.pop_front() {
        // If we reach the destination, return the distance
        if node == destination {
            return distance.get(&node).cloned(); // Return distance to destination
        }

        // use references instead of copies to avoid issues with the borrow checker
        // NOTE: no need to update distances since each node will be discovered at its shortest
        // distance when added to the frontier
        // i.e., each node within n of the root is discovered on the nth pass
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(neighbor) {
                    // mark neighbor as visited, enqueue it, and update its distance
                    e.insert(true);
                    distance.insert(neighbor, distance[&node] + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    None // If destination is not reachable, return None 
    // this is why we return Option<i32>, in Rust this allows us
    // to return "Null"
}
