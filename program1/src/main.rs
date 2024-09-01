/**
 * John Birchwood
 * Program 1 - Binary Search
 */

use std::io::{self, Write, BufRead};

// Macro for sending to stderr
macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

/**
 * Binary Search algorithm
 */
fn binary_search(array: &Vec<String>, length: usize, target: i32) -> () {
    let mut start_index = 0;
    let mut end_index = length - 1;
    
    // If the start index ever exceeds the end index, it means the search
    // was unsuccessful and the search terminates
    while start_index <= end_index {
        // Calculate middle of slice
        let midpoint:usize = (start_index + end_index) / 2; // midpoint
        // Convert the string received from stdin to an i32. This is done here so we only run that 
        // conversion from string to i32 is only done for values we are checking, as opposed to 
        // converting the whole array first (O(n)) then iterating through it.
        let value_at_midpoint = match array[midpoint].parse::<i32>() {
            Ok(val) => val,
            Err(e) => {
                println_stderr!("Unable to parse size from argument: {}", e);
                return;
            }
        };
        // Case midpoint equals target
        if target == value_at_midpoint {
            println!("{}", midpoint);
            return // return if found
        }
        // Case target is less than midpoint
        else if target < value_at_midpoint {
            end_index = midpoint - 1;
        }
        // Case target is greater than midpoint
        else if target > value_at_midpoint {
            start_index = midpoint + 1;
        }
    }
    // if the while loop is ever exited, the target was not found
    println!("Not found")
}

fn main() {
    // Initialize an empty, mutable vector to store the input in
    let mut args: Vec<String> = vec![];

    // Get input from standard in
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        args.push(line);
    }

    // length of the sorted array
    let string_len_sorted_array = &args[0];

    // Rust type/null checking is tricky. It all requires this "Ok:Err" funky syntax
    let sorted_size = match string_len_sorted_array.parse::<usize>() {
        Ok(val) => val,
        Err(e) => {
            println_stderr!("Unable to parse size from argument: {}", e);
            return;
        }
    };

    // make sure there are as many following numbers as the first line suggested, requires this funky "Some/None" syntax
    let _arg = match args.get(sorted_size + 1) {
        Some(val) => val,
        None => {
            println_stderr!("First number does not match size of file.");
            return;
        }
    };

    // Size of query array
    let string_query_size = &args[sorted_size + 1];

    
    let query_size = match string_query_size.parse::<usize>() {
        Ok(val) => val,
        Err(e) => {
            println_stderr!("Unable to parse size from argument: {}", e);
            return;
        }
    };

    // Make sure there are as many following targets as the size suggested
    let _arg_2 = match args.get(sorted_size + query_size) {
        Some(val) => val,
        None => {
            println_stderr!("Number of queries inconsistent with file size.");
            return;
        }
    };

    let query_start_index = sorted_size + 2;

    // run binary search for each target
    for str_target in &args[query_start_index..query_start_index+query_size] {
        let target = match str_target.parse::<i32>() {
            Ok(val) => val,
            Err(e) => {
                println_stderr!("Unable to parse size from argument: {}", e);
                return;
            }
        };
        binary_search(&args[1..sorted_size+2].to_vec(), sorted_size, target);
    }

}
