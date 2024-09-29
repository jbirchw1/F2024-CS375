/**
 * John Birchwood, bone fide Rustacean
 * jbirchw1@binghamton.edu
 * Program 4 - Binary Search Tree
 */

use std::io::{self, BufRead};

mod binary_search_tree;

fn collect_input() -> Vec<i32> {
    // In rust, command line arguments can be collected from std::env,
    // while input provided via file redirection is collected from stdin

    // Read integers from stdin
    let stdin = io::stdin();
    let unsorted_array: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok()) // filter out any lines that couldn't be read
        .filter_map(|line| line.trim().parse().ok()) // convert each line to an i32
        .collect();

    unsorted_array
}

fn parse_input(raw_in: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // get first element 
    let tree_vec_size : usize = <i32 as TryInto<usize>>::try_into(raw_in[0]).unwrap() + 1;
    // slice and convert to vector
    let tree_vec : Vec<i32> = (&raw_in[1..tree_vec_size]).to_vec();
    // remaining elements are deletions
    let deletion_vec_start : usize = tree_vec_size + 1;
    let deletion_vec : Vec<i32> = (&raw_in[deletion_vec_start..]).to_vec();
    (tree_vec, deletion_vec)
}

fn main() {
    // collect and parse input into the input vector and items to delete
    let (input, deletions) = parse_input(collect_input());
    // create binary search tree
    let mut bst = binary_search_tree::BinarySearchTree::from_vec(input);
    // delete each item in the deletions vector
    for item_to_remove in deletions {
        bst.delete(item_to_remove);
    }
    // print bst in order
    bst.in_order_traversal();
}
