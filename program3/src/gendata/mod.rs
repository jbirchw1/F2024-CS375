use rand::seq::SliceRandom;
use rand::Rng;

// Simple function that generates n random integers
// and scrambles them.
// NOTE: The structure of this project makes for some weird
// imports and crate usage. To avoid any warnings at compile time,
// I have disabled the warning that warns against unused functions.
// Furthermore, the tests will not compile if the `mod gendata;` line
// is not included in the `main.rs` file.
#[allow(dead_code)]
pub fn generate_random_list(n: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = (1..n)
        .collect::<Vec<i32>>()
        .iter()
        .chain([0].iter())
        .copied()
        .collect();
    numbers.shuffle(&mut rand::thread_rng());
    numbers
}

// similar to the previous function, but instead of
// enumerating a list from 1 to n, it generates a random
// number between 0 and i32 MAX for each n. Results in
// larger, more unpredictable numbers.
#[allow(dead_code)]
pub fn generate_max_random_list(n: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..n {
        let num: i32 = rand::thread_rng().gen_range(0..i32::MAX);
        numbers.push(num);
    }
    numbers
}
