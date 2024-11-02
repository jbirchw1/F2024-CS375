//! Module for Prim's minimum spanning tree algorithm.
//! Contains the priority_queue module as well. See this module
//! for further details on time complexity for the Prim's Algorithm.

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
/// mod prim;
/// 
/// let my_graph = prim::build_weighted_directed_graph_from_stdin();
/// ```
/// 
/// # Time Complexity
/// <div class="warning">This function operates in 
/// O(V) time complexity.</div>
pub fn build_weighted_undirected_graph_from_stdin() -> (HashMap<i32, Vec<(i32, f64)>>, i32) {
    let mut graph: HashMap<i32, Vec<(i32, f64)>> = HashMap::new();
    let stdin = io::stdin();

    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let mut sizes = line1.split_whitespace();
    let num_vertices: i32 = sizes.next().unwrap().parse().unwrap();
    let _num_edges: i32 = sizes.next().unwrap().parse().unwrap();

    // Read each line of input
    for line in stdin.lock().lines() {
        let input = line.unwrap();

        // Parse the edge
        let mut edge = input.split_whitespace();
        let u: i32 = edge.next().unwrap().parse().unwrap();
        let v: i32 = edge.next().unwrap().parse().unwrap();
        let dist: f64 = edge.next().unwrap().parse().unwrap();

        // if u exists in the hash map, insert v into its' vector
        // if not, create u and then add v
        graph.entry(u).or_insert_with(Vec::new).push((v, dist));
        // also, add the opposite edge
        graph.entry(v).or_insert_with(Vec::new).push((u, dist));
    }

    (graph, num_vertices) // Return ownership of the graph
}

pub fn determine_minimum_spanning_tree_cost(
    graph: &HashMap<i32, Vec<(i32, f64)>>,
    num_vertices: i32) {

    // get distances and predeccessors from Prim's algorithm
    let minimum_spanning_tree = prim(graph, num_vertices);

    let mut total_distance : f64 = 0.0;
    for (_key, value) in minimum_spanning_tree {
        println!("{}:{}", _key, value);
        total_distance += value;
    }
    println!("Total Length: {}", total_distance);
}

pub fn prim(
    graph: &HashMap<i32, Vec<(i32, f64)>>,
    num_vertices: i32 ) -> HashMap<i32, f64> {
    let mut mst: HashMap<i32, f64> = HashMap::new();  // Minimum edge weight to the MST
    let mut pq = priority_queue::PriorityQueue::new();  // Priority queue to select the next minimum edge

    // Start with an arbitrary node (e.g., node 0)
    let start_node = 0;
    mst.insert(start_node, 0.0);
    pq.insert_with_priority(start_node, 0.0);

    // THIS is Dijkstra's algorithm
    // pop lowest distance off priority queue
    while mst.len() < num_vertices.try_into().unwrap() {
        let current = if let Some(current) = pq.pop() { current } else { return mst; };
        // add current to mst
        println!("Current = {}:{}", current.get_key(), current.get_distance());
        mst.insert(current.get_key(), current.get_distance());
        // iterate through each neighbor, if it exists
        if let Some(neighbors) = graph.get(&current.get_key()) {
            for &neighbor in neighbors {
                println!("Considering adding neighbor: {}:{}", &neighbor.0, &neighbor.1);
                if !mst.contains_key(&neighbor.0) {
                    // if the mst does not yet contain the vertex associated with the tail end of the edge
                    // add it to the priority queue
                    println!("Inserting neighbor: {}:{}", &neighbor.0, &neighbor.1);
                    pq.insert_with_priority(neighbor.0, neighbor.1);
                }
            }
        }
    }

    // Return the mst, with distances for each vertex
    mst
}