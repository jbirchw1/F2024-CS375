/**
 * John Birchwood, bona fide Rustacean
 * jbirchw1@binghamton.edu
 * Program 3 - Heap Sort
 */

/**
 * TODO: Include comments on time complexity
 * * Or, check README.md
 */

// imports heap module (see heap/mod.rs)
mod heap;

// Required libraries for input parsing
use std::env;
use std::io::{self, BufRead};

/**
 * This function takes no parameters and returns the start output index, the end output index,
 * and the unsorted input data gathered from stdin.
 * Returns the entire unsorted input array in the form of an i32 vector
 * and returns the indices as uints of type usize, useful for automatic
 * error checking and type safety.
 * If no index arguments are provided on the command line, the function
 * will assign them to the start and end of the input (max size).
 */
fn collect_and_parse_input() -> (usize, usize, Vec<i32>) {
    // In rust, command line arguments can be collected from std::env,
    // while input provided via file redirection is collected from stdin

    // skip first argument, and collect the optional indeces
    let args: Vec<String> = env::args().skip(1).collect();

    // Set to 0 and 1 for initialization purposes.
    // In practice, these will virtually always be set correctly later in the program.
    let mut start: usize = 0;
    let mut end: usize = 1;

    // Read integers from stdin
    // Important: must read in size of unsorted array here in order to return appropriate
    // ending index in the event there are no command line arguments provided
    let stdin = io::stdin();
    let unsorted_array: Vec<i32> = stdin
        .lock()
        .lines()
        .skip(1)
        .filter_map(|line| line.ok()) // filter out any lines that couldn't be read
        .filter_map(|line| line.trim().parse().ok()) // convert each line to an i32
        .collect();

    // handles errors in command line input and appropriately assigns start and end indices for printing
    if args.len() == 1 {
        println!("ID-10-t Error: Not enough indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
    } else if args.len() == 2 {
        // case in which the user wishes to output only the items between two vertices
        start = match args[0].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0); // Force quit program. Will not behave as expected if it continues
            }
        };
        end = match args[1].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0);
            }
        };
        // User error checking
        if start > unsorted_array.len() {
            println!("ID-10-t Error: starting index {} out of bounds.", start);
            std::process::exit(0);
        } else if end > unsorted_array.len() {
            println!("ID-10-t Error: ending index {} out of bounds.", end);
            std::process::exit(0);
        }
    } else if args.len() > 2 {
        println!("ID-10-t Error: Too many indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
        std::process::exit(0);
    } else {
        // case in which no input was provided => print entire array
        start = 0;
        end = unsorted_array.len();
    }

    // return values (weird syntax, I know)
    (start, end, unsorted_array)
}

// Standard printing array function
fn print_array(start: usize, end: usize, array: Vec<i32>) {
    // print size
    let subset_eq_size = end - start; // size of output
    println!("{}", subset_eq_size);
    // Print array
    for each_integer in &array[start..end] {
        println!("{}", each_integer);
    }
}
 
fn main() {
    // collect and parse input
    let (start_print_index, end_print_index, unsorted_array) = collect_and_parse_input();
    // sort array (see heap/mod.rs)
    let sorted_array = heap::sort(unsorted_array);
    // print
    print_array(start_print_index, end_print_index, sorted_array);
}
