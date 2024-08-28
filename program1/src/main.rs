/**
 * John Birchwood
 * Program 1 - Binary Search
 */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/**
 * This function reads text from the input file into a Buffered Reader
 * Although unneccessary, it will panic if the input read is not valid.
 * The output is returned from the function as a Vector of strings.
 */
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/**
 * This function takes the Vector of strings previously compiled and converts
 * it to usable vectors in the form of 32-bit integers.
 * It returns 2 i32 Vectors, the sorted list of numbers and the list of targets
 * to be searched for.
 */
fn convert_and_parse_input(strings: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    // Take the first element and use it as the length of the sorted list
    let array_size = match strings[0].trim().parse::<i32>() {
        Ok(num) => {
            num
        },
        Err(_) => {
            eprintln!("Failed to parse the array size as i32: {}", strings[0]);
            return (Vec::new(), Vec::new()); // Return empty vectors on error
        }
    };

    // Create new i32 Vector with size computed above.
    let mut sorted_numbers: Vec<i32> = Vec::with_capacity(array_size as usize);

    // For each number within the range computed with the size, convert from string to int and 
    // push to the Vector.
    for (index, line) in strings.iter().enumerate().skip(1).take((array_size) as usize) {
        match line.trim().parse::<i32>() {
            Ok(num) => sorted_numbers.push(num),
            Err(_) => eprintln!("Failed to parse integer at line {} as i32: {}", 1 + index, line),
        }
    }

    // Take the next index after the list of sorted numbers and use that for the 
    // number of search targets.
    let query_size = match strings[(array_size + 1) as usize].trim().parse::<i32>() {
        Ok(num) => {
            num
        },
        Err(_) => {
            eprintln!("Failed to parse the query size as i32: {}", strings[0]);
            return (Vec::new(), Vec::new()) ; // Return empty vectors on error
        }
    };

    // New Vector for the queries
    let mut queries: Vec<i32> = Vec::with_capacity(query_size as usize);

    // Skip all the elements that are not in the target vector
    // (sorted vector size) + (vector array) + (target vector size) => sorted_vector + 2
    for (index, line) in strings.iter().enumerate().skip((array_size + 2) as usize).take((query_size) as usize) {
        match line.trim().parse::<i32>() {
            Ok(num) => queries.push(num),
            Err(_) => eprintln!("Failed to parse query at line {} as i32: {}", 1 + index, line),
        }
    }

    // This is how you return in Rust. Unintutive for me, since I am mainly a C++ programmer.
    (sorted_numbers, queries)
}

// !! This is the binary search algorithm
fn binary_search(array: &Vec<i32>, length: usize, target: i32) -> () {
    let mut start_index = 0;
    let mut end_index = length - 1;
    
    while start_index <= end_index {
        let midpoint:usize = (start_index + end_index) / 2;
        if target == array[midpoint] {
            println!("{}", midpoint);
            return // return if found
        }
        else if target < array[midpoint] {
            end_index = midpoint - 1;
        }
        else if target > array[midpoint] {
            start_index = midpoint + 1;
        }
    }
    // if the while loop is ever exited, the target was not found
    println!("Not found")
}

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Set the input filepath to the value of the first command line argument
    // * NOTE: The first command line (or the 0th) is the path to the executable, consistent w/ C
    // behavior 
    let file_path = &args[1];
    let raw_input = lines_from_file(file_path);
    let (sorted_numbers, queries) = convert_and_parse_input(raw_input);

    // Search for each target
    for target in queries.iter() {
        // ! Must use a copy (& notation) to pass by value
        // Also, need to dereference target value
        binary_search(&sorted_numbers, sorted_numbers.len().try_into().unwrap(), *target);
    }

}
