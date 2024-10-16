/**
 * John Birchwood, bone fide Rustacean
 * jbirchw1@binghamton.edu
 * Program 5 - Breadth First Search
 */
use std::env;

mod dijkstra;

/**
 * Function to get the start and end vertices from the command line
 * Returns (start, end) as i32s.
 */
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

fn main() {
    let (start, end) = parse_args();
    let my_graph = dijkstra::build_graph_from_stdin();
    dijkstra::print_graph(&my_graph);
    dijkstra::shortest_path();
}
