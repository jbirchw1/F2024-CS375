#![warn(missing_docs)]

//! Binghamton University - Fall 2024 Design and Analysis of Algorithms
//! 
//! Program 7: Prim's Algorithm - John Birchwood
//! 
//! Please click any modules/functions to read more.


/**
 * John Birchwood, bone fide Rustacean
 * jbirchw1@binghamton.edu
 * Program 7 - Prim's Algorithm
 */

mod prim;


/// Function to get the start and end vertices from the command line
/// Returns (start, end) as i32s.
/// 
/// # Examples 
/// ```
/// let (start, end) = parse_args();
/// ```

/// Driver code
fn main() {
    let (my_graph, num_vertices) = prim::build_weighted_undirected_graph_from_stdin();
    prim::determine_minimum_spanning_tree_cost(&my_graph, num_vertices);
    
}
