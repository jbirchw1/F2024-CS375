use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/**
 * TODO: Add stuff
 */
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/**
 * TODO: Add stuff
 */
fn convert_and_parse_input(strings: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let array_size = match strings[0].trim().parse::<i32>() {
        Ok(num) => {
            // println!("Read size: {}", num);
            num
        },
        Err(_) => {
            eprintln!("Failed to parse the array size as i32: {}", strings[0]);
            return (Vec::new(), Vec::new()); // Return an empty vector on error
        }
    };

    let mut sorted_numbers: Vec<i32> = Vec::with_capacity(array_size as usize);

    for (index, line) in strings.iter().enumerate().skip(1).take((array_size) as usize) {
        match line.trim().parse::<i32>() {
            Ok(num) => sorted_numbers.push(num),
            Err(_) => eprintln!("Failed to parse integer at line {} as i32: {}", 1 + index, line),
        }
    }

    let query_size = match strings[(array_size + 1) as usize].trim().parse::<i32>() {
        Ok(num) => {
            // println!("Read query size: {}", num);
            num
        },
        Err(_) => {
            eprintln!("Failed to parse the query size as i32: {}", strings[0]);
            return (Vec::new(), Vec::new()) ; // Return an empty vector on error
        }
    };

    let mut queries: Vec<i32> = Vec::with_capacity(query_size as usize);

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
            return
        }
        else if target < array[midpoint] {
            end_index = midpoint - 1;
        }
        else if target > array[midpoint] {
            start_index = midpoint + 1;
        }
    }
    println!("Not Found")
}

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Set the input filepath to the value of the first command line argument
    // * NOTE: The first command line (or the 0th) is the path to the executable, consistent w/ C
    // behavior 
    // TODO: Change these names
    let file_path = &args[1];
    let raw_input = lines_from_file(file_path);
    let (sorted_numbers, queries) = convert_and_parse_input(raw_input);

    for target in queries.iter() {
        binary_search(&sorted_numbers, sorted_numbers.len().try_into().unwrap(), *target);
    }

}
