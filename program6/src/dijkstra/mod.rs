// module for disktra's shortest path algorithm

use std::collections::{HashMap};
use std::io::{self, BufRead};
mod priority_queue;

pub fn print_graph(graph: &HashMap<i32, Vec<(i32, i32)>>) {
    println!("adjacency list for each edge:");
    for (node, neighbors) in graph {
        println!("{}: {:?}", node, neighbors);
    }
}

// TODO: build helper function to get rid of potential duplicate edges
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

pub fn shortest_path(
    start: i32,
    destination: i32,
    graph: &HashMap<i32, Vec<(i32, i32)>>,
) -> Option<i32> {
    /** let mut visited = HashMap::new(); // visited nodes 
    let mut distance = HashMap::new(); // distances from start node
    // TODO: heap
    // add values as tuples, using the distance as the comparison value
    // may need to use f32/f64 to represent f32::INFINITY
    let mut queue : Vec<priority_queue::Vertex>; // frontier queue

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
    **/

    // ! testing 
    let mut queue = priority_queue::PriorityQueue::new();
    queue.insert(4);
    queue.insert(5);
    queue.insert_with_priority(1, 1.0); // kinda the root
    queue.decrease_key(4, 0.0);
    let curr = queue.pop();
    let l : usize = queue.length();
    println!("Size of PQ is {}", l);

    None
}