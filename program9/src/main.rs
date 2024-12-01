//! Binghamton University - Fall 2024 Design and Analysis of Algorithms
//!
//! Program 8: Longest Common Substring  - John Birchwood

use std::io::{self, BufRead};
mod knapsack;

// Get all items and associated values + weights
fn build_input() -> (Vec<u32>, Vec<u32>, u32) {
    let mut values = Vec::new();
    let mut weights = Vec::new();

    let stdin = io::stdin();

    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let mut sizes = line1.split_whitespace();
    let _num_items: u32 = sizes.next().unwrap().parse().unwrap();
    let max_weight: u32 = sizes.next().unwrap().parse().unwrap();

    for line in stdin.lock().lines() {
        let input = line.unwrap();

        // Parse the item
        let mut item = input.split_whitespace();
        let w: u32 = item.next().unwrap().parse().unwrap();
        let v: u32 = item.next().unwrap().parse().unwrap();

        values.push(v);
        weights.push(w);
    }

    return (values, weights, max_weight);
}

fn main() {
    let (values, weights, max_weight) = build_input();
    knapsack::determine_and_print_loot(max_weight, values, weights);
}
