//! Binghamton University - Fall 2024 Design and Analysis of Algorithms
//! 
//! Program 8: Longest Common Substring  - John Birchwood

use std::io;
mod lcs;

fn read_strings_from_stdin() -> (Vec<char>, Vec<char>) {
    let mut first = String::new();
    let mut second = String::new();

    io::stdin().read_line(&mut first).expect("Failed to read line");
    io::stdin().read_line(&mut second).expect("Failed to read line");

    let string1: Vec<char> = first.chars().collect();
    let string2: Vec<char> = second.chars().collect();

    (string1, string2)
}

fn main() {
    // get strings
    let (mut string1, string2) = read_strings_from_stdin();
    let directions = lcs::compute_lcs(&string1, &string2); // calculate lcs
    // add a spacer so that the indices of the string and tables line up
    string1.insert(0, ' ');
    let m = string1.len();
    let n = string2.len();
    // print lcs
    lcs::print_output(directions, &string1, m-2, n);
}
