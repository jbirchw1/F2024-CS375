//! Module for Dijkstra's shortest path algorithm.
//! Contains the priority_queue module as well. See this module
//! for further details on time complexity for the Dijkstra's Algorithm.

use std::collections::{HashMap};
use std::io::{self, BufRead};
mod priority_queue;

/// Reads graph input from stdin and creates a hashmap representation
/// to use in weighted graph applications.
/// 
/// The input file is to start with a line `x y`, which describes the 
/// number of nodes and number of edges.
/// Each following line will describe a weighted, directed edge, of the form
/// `u v d`, where the edge e goes from u -> v with weight d.
/// 
/// # Examples
/// Simply call the function with no arguments, e.g.
/// ```
/// mod dijkstra;
/// 
/// let my_graph = dijkstra::build_weighted_directed_graph_from_stdin();
/// ```
/// 
/// # Time Complexity
/// <div class="warning">This function operates in 
/// O(V) time complexity.</div>
pub fn build_weighted_directed_graph_from_stdin() -> HashMap<i32, Vec<(i32, f64)>> {
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

/// Prints the shortest path from a start vertex to an end vertex over a 
/// given graph.
/// 
/// Start and end vertices are to be passed as i32, and the graph is to be passed
/// as a reference to a weighted graph hashmap representation, as returned from build_weighted_directed_graph_from_stdin().
/// 
/// If a path is not found between start and end vertices, it will print "not connected"
/// If a path is found, it will print the distance followed by the shortest path.
/// E.g.
/// ```
/// Distance: 14
/// 1 -> 2 -> 3 -> 4 -> 5
/// ```
/// 
/// # Examples
/// ```
/// mod dijkstra;
/// 
/// let (source, destination) = (0, 5)
/// 
/// let my_graph = dijkstra::build_weighted_directed_graph_from_stdin();
/// dijkstra::print_shortest_path(source, destination, &my_graph);
/// ```
/// 
/// # Time Complexity
/// <div class="warning">This function operates in 
/// O(VlogV) time complexity.</div>
/// Note that this is only due to the call to dijkstra(). 
/// Otherwise, the function operates in O(V) time.
pub fn print_shortest_path(
    start: i32,
    destination: i32,
    graph: &HashMap<i32, Vec<(i32, f64)>> ) {

    // take care of the O(1) case in which the start and end 
    // vertices are the same.
    if start == destination {
        println!("Distance: 0");
        println!("Path:  {}", start);
        return;
    }

    // get distances and predeccessors from dijkstra's algorithm
    let (distances, predecessors) = dijkstra(graph, start);

    // if node was not discovered
    if distances.get(&destination).copied().unwrap_or(f64::INFINITY) == f64::INFINITY {
        println!("not connected");
        return;
    }

    // get distance
    let distance_to_destination = distances.get(&destination).copied().unwrap();
    println!("Distance: {}", distance_to_destination);

    // print path by traversing predeccessors in reverse order
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

    print!("Path:  ");
    while stack.len() > 0 {
        let curr = stack.pop();
        print!("{} ", curr.unwrap());
    }
    println!("");
}

/// Dijkstra's algorithm for all shortest paths from a start vertex.
/// 
/// Reads a reference to a hashmap weighted graph representation (as 
/// described in build_weighted_directed_graph_from_stdin()) and 
/// returns the shortest distance to all vertices from the start vertex (second argument).
/// 
/// Returns 2 HashMaps: one for distances and another for each vertex's predecessor.
/// This function can be called on its own, or more typically from inside the print_shortest_path function.
/// 
/// # Examples
/// ```
/// mod dijkstra;
/// 
/// let source = 0;
/// 
/// let my_graph = dijkstra::build_weighted_directed_graph_from_stdin();
/// let distances, predecessors = dijkstra::dijkstra(&my_graph, source);
/// ``` 
/// 
/// # Time Complexity
/// <div class="warning">This function operates in 
/// O(VlogV) time complexity.</div>
pub fn dijkstra(
    graph: &HashMap<i32, 
    Vec<(i32, f64)>>, 
    start: i32,
) -> (HashMap<i32, f64>, HashMap<i32, i32>) {

    let mut distances: HashMap<i32, f64> = HashMap::new();  // Stores the shortest distance to each node
    let mut predecessors: HashMap<i32, i32> = HashMap::new();  // Stores the predeccessor for each node

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

    // Start node with distance 0
    distances.insert(start, 0.0);
    pq.insert_with_priority(start, 0.0);  

    // THIS is Dijkstra's algorithm
    // pop lowest distance off priority queue
    while let Some(current) = pq.pop() {
        // iterate through each neighbor, if it exists
        if let Some(neighbors) = graph.get(&current.get_key()) {
            for &neighbor in neighbors {
                // calculate old and new distances
                let prior = distances.get(&neighbor.0).copied().unwrap();
                let post = &current.get_distance() + &neighbor.1;
                if prior > post { // if a better distance has been discovered
                    // update distance map, predeccessor map, and relax edge in priority queue (or add it)
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

    // return distances and each vertex's predecessor
    (distances, predecessors)
}