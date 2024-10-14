use std::env;

mod bfs;

fn main() {
    let debug = false;

    // create graph from input
    let graph = bfs::build_graph_from_stdin();
    // print graph if wanted
    if debug {
        bfs::print_graph(&graph);
    }

    // get start and end vertices from command line
    let args: Vec<String> = env::args().skip(1).collect();
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

    // print correct output based on return value of bfs algorithm
    match bfs::breadth_first_search(start, end, &graph) {
        Some(dist) => println!("{}", dist),
        None => println!("not connected"),
    }
}
