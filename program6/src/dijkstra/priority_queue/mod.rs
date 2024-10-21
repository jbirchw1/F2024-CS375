// module for the priority queue
use std::collections::{HashMap};

pub struct Vertex {
    key: i32,
    distance: f64,
}

pub struct PriorityQueue {
    heap: Vec<Vertex>,
    lookup_table: HashMap<i32, usize>,
}


impl Vertex {
    pub fn new(k: i32) -> Self {
        Self{
            key: k,
            distance: f64::INFINITY,
        }
    }

    pub fn new_with_priority(k: i32, d: f64) -> Self {
        Self{
            key: k,
            distance: d,
        }
    }
}

impl PriorityQueue {
    pub fn new() -> Self {
        let vec: Vec<Vertex> = Vec::new();
        let hm: HashMap<i32, usize> = HashMap::new();
        Self{
            heap: vec,
            lookup_table: hm,
        }
    }

    pub fn length(&self) -> usize {
        self.heap.len()
    }

    // Swaps indices in heap and updates lookup table
    fn swap(&mut self, index1: usize, index2: usize) {
        self.heap.swap(index1, index2);
        self.lookup_table.insert(self.heap[index1].key, index1);
        self.lookup_table.insert(self.heap[index2].key, index2);
    }

    fn percolate_up(&mut self, key: i32) {
        let to_percolate = self.lookup_table.get(&key).copied().unwrap();
        for (key, value) in &self.lookup_table {
            println!("{}: {}", key, value);
        }

        println!("index to percolate = {}", to_percolate);
        let mut index = to_percolate;
        while index > 0 {
            let parent = (index - 1) / 2;
            println!("index now equals {}", index);
            println!("value at index {} is {}", index, self.heap[index].key);
            if(self.heap[index].key < self.heap[parent].key) {
                println!("swapping");
                self.swap(index, parent);
            }
            println!("value at the same index {} is now {}", index, self.heap[index].key);
            index = parent;
        }

        for (key, value) in &self.lookup_table {
            println!("{}: {}", key, value);
        }
    }

    pub fn insert(&mut self, vertex: i32) {
        let v = Vertex::new(vertex);
        // No need to percolate up, because when it is inserted into the back of the 
        // heap it has distance infinity regardless
        self.heap.push(v);
        let index = self.heap.iter().position(|n| n.key == vertex);
        self.lookup_table.insert(vertex, index.unwrap());
    }

    pub fn insert_with_priority(&mut self, vertex: i32, weight: f64) {
        let v = Vertex::new_with_priority(vertex, weight);
        self.heap.push(v);
        let index = self.heap.iter().position(|n| n.key == vertex);
        self.lookup_table.insert(vertex, index.unwrap());
        self.percolate_up(vertex);
    }

    pub fn decrease_key(&mut self) {
        unimplemented!();
    } 
}
/**
 * Function that percolates down through the heap the value at given index
 * Takes as input the i32 heap vector and the index at which to start sifting
 * Returns the re-heapified i32 vector
 */
fn percolate_down(mut index: usize, mut heap: Vec<i32>) -> Vec<i32> {
    let length = heap.len();

    // While left child exists
    while 2 * index + 1 < length {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut curr_smallest = index;

        // determine if either of the two children is smaller than the current index
        // if so, it will set curr_smallest to the smaller of the two children
        if left < length && heap[left] < heap[curr_smallest] {
            curr_smallest = left;
        }
        if right < length && heap[right] < heap[curr_smallest] {
            curr_smallest = right;
        }

        // if current index is smaller than both children, this subtree rooted
        // at the current index is a heap
        // NOTE: This is why we must work bottom up, since it only works under the
        // assumption that the remaining subtree is a heap.
        if curr_smallest == index {
            return heap;
        }

        // native rust swap function to avoid nasty "temp" syntax
        heap.swap(index, curr_smallest);
        // set index to the place we just swapped with
        index = curr_smallest;
    }

    heap
}

/**
 * Function to extract the minimum value from the heap and re-heapify
 * the remaining vector
 * Takes the i32 vector heap as input
 * Returns the minimum as i32 and the remaining heap as i32 vector
 */
fn extract_min(mut heap: Vec<i32>) -> (i32, Vec<i32>) {
    // panic if trying to extract from empty vector
    if heap.is_empty() {
        // ? maybe edit this s.t. this is what returns when heap is empty? return option?
        panic!("Error ID-10-t: cannot extract minimum from empty heap");
    }

    // By the definition of a heap, the root should be the smallest
    let minimum = heap[0];

    // if length is one, simply return an empty vector and the minimum
    if heap.len() == 1 {
        heap.pop();
        return (minimum, heap);
    }

    // if non-singleton, swap last and first element then shrink vector by 1
    let last_index = heap.len() - 1;
    heap.swap(0, last_index);
    heap.pop();

    // percolate the root value down
    // obviously, this is done to preserve the complete tree structure of the heap
    let resized_heap = percolate_down(0, heap);

    // return the minimum and the re-heapified heap
    (minimum, resized_heap)
}

/**
 * For each non-leaf value, percolate down the subtree until the
 * condition (heap[index] < both children) is satisfied.
 * Takes an unsorted i32 vector as input.
 * Returns a heapified i32 vector.
 */
fn heapify(mut heap: Vec<i32>) -> Vec<i32> {
    // calculate index of first non-leaf node, convert from usize -> i32
    let mut index: i32 = (heap.len() / 2 - 1).try_into().unwrap();
    while index >= 0 {
        // percolate down at each index,
        // must convert from i32 since vectors can only be indexed by usize
        heap = percolate_down(index.try_into().unwrap(), heap);
        // important that index remains an i32 so this doesn't panic
        index -= 1;
    }
    heap
}

/**
 * Public function allowing the user to build a min heap using
 * the private heapify function.
 * Takes an unsorted i32 vector as input.
 * Returns the heapified array as an i32 vector.
 */
// My dream is that if I ever need to expand the program to
// max heaps as well, I'll be able to adjust heapify to
// support both min and max heaps, and simply create another
// public function, namely build_max_heap().
pub fn build_min_heap(unsorted_heap: Vec<i32>) -> Vec<i32> {
    heapify(unsorted_heap)
}