// module for disktra's shortest path algorithm

use std::collections::{HashMap};
use std::io::{self, BufRead};
mod priority_queue;

// TODO: build helper function to get rid of potential duplicate edges
// -> not strictly neccessary, but can help cut down on runtime 
pub fn build_graph_from_stdin() -> HashMap<i32, Vec<(i32, f64)>> {
    let mut graph: HashMap<i32, Vec<(i32, f64)>> = HashMap::new();
    let stdin = io::stdin();

    // Read each line of input
    for line in stdin.lock().lines().skip(1) {
        let input = line.unwrap();

        // Parse the edge
        let mut edge = input.split_whitespace();
        let u: i32 = edge.next().unwrap().parse().unwrap();
        let v: i32 = edge.next().unwrap().parse().unwrap();
        let dist: f64 = edge.next().unwrap().parse().unwrap();

        // if u exists in the hash map, insert v into its' vector
        // if not, create u and then add v
        graph.entry(u).or_insert_with(Vec::new).push((v, dist));
    }

    graph // Return ownership of the graph
}

pub fn print_shortest_path(
    start: i32,
    destination: i32,
    graph: &HashMap<i32, Vec<(i32, f64)>> ) {

    let (distances, predecessors) = dijkstra(graph, start);

    if distances.get(&destination).copied().unwrap_or(f64::INFINITY) == f64::INFINITY {
        println!("not connected");
        return;
    }

    let distance_to_destination = distances.get(&destination).copied().unwrap();
    println!("Distance: {}", distance_to_destination);

    let mut stack = Vec::new();
    stack.push(destination);
    let mut prior = predecessors.get(&destination).copied().unwrap(); 
    loop {
        stack.push(prior);
        if prior == start {
            break;
        }
        prior = predecessors.get(&prior).copied().unwrap(); 
    }

    while stack.len() > 0 {
        let curr = stack.pop();
        if stack.len() > 0 {
            print!("{} -> ", curr.unwrap());
        }
        else {
            println!("{}", curr.unwrap());
        }
    }
}

fn dijkstra(
    graph: &HashMap<i32, 
    Vec<(i32, f64)>>, 
    start: i32,
) -> (HashMap<i32, f64>, HashMap<i32, i32>) {

    let mut distances: HashMap<i32, f64> = HashMap::new();  // Stores the shortest distance to each node
    let mut predecessors: HashMap<i32, i32> = HashMap::new();  // Stores the path for each node

    // Initialize the priority queue
    let mut pq = priority_queue::PriorityQueue::new();

    // Initialize distances to infinity and start node to 0
    for &vertex in graph.keys() {
        distances.insert(vertex, f64::INFINITY);
        pq.insert_with_priority(vertex, f64::INFINITY);
        // little hack-y, but allows you to add nodes that aren't nececcarily "u" in a directed edge
        let destinations = graph.get(&vertex).unwrap();
        for d in destinations {
            distances.insert(d.0, f64::INFINITY);
            // dont need to insert to pq here since any vertex that does not have
            // an edge start will not have any neighbors
        }

    }
    distances.insert(start, 0.0);

    pq.insert_with_priority(start, 0.0);  // Start node with distance 0

    while let Some(current) = pq.pop() {
        if let Some(neighbors) = graph.get(&current.get_key()) {
            for &neighbor in neighbors {
                let prior = distances.get(&neighbor.0).copied().unwrap();
                let post = &current.get_distance() + &neighbor.1;
                if prior > post {
                    distances.insert(neighbor.0, post);
                    predecessors.insert(neighbor.0, current.get_key());
                    if pq.contains_key(neighbor.0) {
                        pq.relax(neighbor.0, post);
                    }
                    else {
                        pq.insert_with_priority(neighbor.0, post);
                    }
                    
                }
            }
        }
    }

    (distances, predecessors)
}