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

use std::env;
mod prim;


/// Function to get the start and end vertices from the command line
/// Returns (start, end) as i32s.
/// 
/// # Examples 
/// ```
/// let (start, end) = parse_args();
/// ```
fn parse_args() -> (i32, i32) {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("ID-10-t Error: Not enough indices entered. Please enter start and end vertices.");
        std::process::exit(0);
    }
    let start: i32 = match args[0].parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0); // Force quit program. Will not behave as expected if it continues
        }
    };
    let end: i32 = match args[1].parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    };
    (start, end)
}

/// Driver code
fn main() {
    let (start, end) = parse_args();
    let my_graph = prim::build_weighted_directed_graph_from_stdin();
    prim::print_shortest_path(start, end, &my_graph);
}
