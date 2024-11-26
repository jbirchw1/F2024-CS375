//! module for the knapsack program w/ memoization

// TODO: Add comments
// TODO: cargo fmt and cargo clippy

use std::cmp::max;

// finds the items included in the knapsack
fn find_selected_items(max_weight: u32, weights: &Vec<u32>, sack: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut selected_items = Vec::new();
    let mut remaining_capacity = max_weight as usize;
    let num_items = weights.len();

    for i in (0..num_items).rev() {
        if i == 0 {
            if sack[i][remaining_capacity] != 0 {
                selected_items.push(i+1);
            }
            break;
        }

        if sack[i][remaining_capacity] != sack[i - 1][remaining_capacity] {
            selected_items.push(i+1); // This item is included
            remaining_capacity -= weights[i] as usize;
        }
    }

    selected_items
}

// recursive helper function
// fills out the memoization table and determines max value
fn recurse(max_weight: u32, values: &Vec<u32>, weights: &Vec<u32>, index: i32, sack: &mut Vec<Vec<i32>>) -> i32 {
    if index < 0 {
        return 0;
    }

    if sack[index as usize][max_weight as usize] != -1 {
        return sack[index as usize][max_weight as usize];
    }

    if weights[index as usize] > max_weight {
        sack[index as usize][max_weight as usize] = recurse(max_weight, values, weights, index-1, sack);
        return sack[index as usize][max_weight as usize];
    }
    // else 
    sack[index as usize][max_weight as usize] = max(
                ((values[index as usize] as i32) + recurse(max_weight - weights[index as usize], values, weights, index-1, sack)) as i32,
                recurse(max_weight, values, weights, index-1, sack)   );
    return sack[index as usize][max_weight as usize];
}

// driver function
// fill out table and determine which items to include
pub fn determine_and_print_loot(max_weight: u32, values: Vec<u32>, weights: Vec<u32>) {
    let num_items = weights.len();

    // 2d array
    let mut sack : Vec<Vec<i32>> = Vec::new();

    // fill every cell with -1 to start
    for i in 0..num_items {
        sack.push(Vec::new());
        for _j in 0..=max_weight {
            sack[i].push(-1);
        }
    }
    
    let solution = recurse(max_weight, &values, &weights, (num_items-1).try_into().unwrap(), &mut sack);
    let mut items = find_selected_items(max_weight, &weights, &sack);

    println!("Total value: {}", solution);
    while let Some(item) = items.pop() {
        println!("Item {}", item);
    }
}

