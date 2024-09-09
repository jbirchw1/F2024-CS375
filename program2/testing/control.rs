use std::env;
use std::io::{self, BufRead};

fn collect_and_parse_input() -> (usize, usize, Vec<i32>) {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut start: usize = 0;
    let mut end: usize = 1;

    let stdin = io::stdin();
    let unsorted_array: Vec<i32> = stdin.lock().lines().skip(1)
        .filter_map(|line| line.ok())  
        .filter_map(|line| line.trim().parse().ok()) 
        .collect();

    if args.len() == 1 {
        println!("ID-10-t Error: Not enough indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
    }
    else if args.len() == 2 {
        start = match args[0].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0);
            }
        };
        end = match args[1].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0);
            }
        };
        if start > unsorted_array.len() {
            println!("ID-10-t Error: starting index {} out of bounds.", start);
            std::process::exit(0);
        }
        else if end > unsorted_array.len() {
            println!("ID-10-t Error: ending index {} out of bounds.", end);
            std::process::exit(0);
        }
    }
    else if args.len() > 2 {
        println!("ID-10-t Error: Too many indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
        std::process::exit(0);
    }
    else {
        start = 0;
        end = unsorted_array.len();
    }

    (start, end, unsorted_array)
}

fn print_array(start: usize, end: usize, array: Vec<i32>) {
    let subset_eq_size = end - start; // size of output
    println!("{}", subset_eq_size);
    for each_integer in &array[start..end] {
        println!("{}", each_integer);
    }
}

fn main() {
    let (start_print_index, end_print_index, mut unsorted_array) = collect_and_parse_input();
    unsorted_array.sort();
    print_array(start_print_index, end_print_index, unsorted_array);
}