// This file has been cut down significantly from program 2.
// It exists simply for benchmarking against heapsort.

use vecshard::ShardExt; // library for O(1) splitting of vectors

/**
 * Takes an unsorted array as input and returns a sorted array,
 * both as type Vec<i32>.
 * Will split the array recursively, using helper function merge() to
 * piece them back together in sorted order.
 */
pub fn sort(unsorted_array: Vec<i32>) -> Vec<i32> {
    // check for the base case (length == 1)
    if unsorted_array.len() <= 1 {
        return unsorted_array; // but really it is sorted (maybe a bad name, i dont know/care)
    }

    // Note: usize automatically sizes to the size of the data type (so for i32, 32 bits)
    // calculate midpoint and split vector at that point using vecshard library (O(1))
    let midpoint: usize = unsorted_array.len() / 2;
    let (unsorted_left_shard, unsorted_right_shard) = unsorted_array.split_inplace_at(midpoint);

    // convert shards back into vectors
    let unsorted_left_array: Vec<i32> = unsorted_left_shard.into();
    let unsorted_right_array: Vec<i32> = unsorted_right_shard.into();

    // recursive step
    let sorted_left_array = sort(unsorted_left_array);
    let sorted_right_array = sort(unsorted_right_array);

    // merge sorted array and return sorted merged vector
    merge(sorted_left_array, sorted_right_array)
}

/**
 * Takes in two sorted arrays as Vec<i32> and merges them into
 * a single Vec<i32>, preserving sorted order.
 * Simply takes the next largest element from either the left
 * or right sub array, and repeats until each subarray is empty.
 */
fn merge(left_array: Vec<i32>, right_array: Vec<i32>) -> Vec<i32> {
    let mut result_vector = Vec::new(); // new array to store result of merge
    let (mut l, mut r) = (0, 0);

    while l < left_array.len() && r < right_array.len() {
        if left_array[l] <= right_array[r] {
            // note: the condition less than or equal makes this a stable algorithm
            result_vector.push(left_array[l]);
            l += 1;
        } else {
            result_vector.push(right_array[r]);
            r += 1;
        }
    }
    // have to take care of the rest of the list if they are of uneven size
    if l < left_array.len() {
        while l < left_array.len() {
            // add tail of left list
            result_vector.push(left_array[l]);
            l += 1;
        }
    } else {
        while r < right_array.len() {
            // add tail of right list
            result_vector.push(right_array[r]);
            r += 1;
        }
    }

    result_vector
}