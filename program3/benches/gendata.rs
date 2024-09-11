use rand::seq::SliceRandom;

// Simple function that generates n random integers
// and scrambles them.
pub fn generate_random_list(n: i32) -> Vec<i32>{
    let mut numbers: Vec<i32> = (0..n)
        .collect::<Vec::<i32>>()
        .iter()
        .chain([0]
            .iter())
            .map(|x| *x)
        .collect();
    numbers.shuffle(&mut rand::thread_rng());
    numbers
}