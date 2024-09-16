// This file exists for purely testing purposes.
// Notice: The benchmark tests exclude the time for 
// parsing input and printing the array, since
// these are exactly the same for both sorts. Therefore
// we simply pass a vector of the appropriate format and
// time the sort itself. This allows for a more direct comparison
// of the two algorithms.
//
// Of course, one could simply run the time command and
// avoid coding all of this benchmarking business. But 
// that's no fun, is it?
mod merge;
mod heap;
mod gendata;

pub fn mergesort(arr: Vec<i32>) {
    merge::sort(arr);
}

pub fn heapsort(arr: Vec<i32>) {
    heap::sort(arr); 
}

pub fn gendata(n: i32) -> Vec<i32> {
    gendata::generate_random_list(n)
}
