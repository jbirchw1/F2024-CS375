//! Module for the PriorityQueue structs and implementations.
//! Also contains the Vertex struct and implementations.
//! 
//! Though the functionality here is entirely contained within the Dijkstra 
//! module, it may be easily expanded via a lib.rs file into much more portable code,
//! were the need ever to arise.

use std::collections::{HashMap};
use std::cmp::Ordering;

/// Struct for vertices.
/// 
/// The PriorityQueue binary min heap is constructed using this 
/// struct to aid in comparisons and manipulation, instead of worrying
/// about some tuple.
pub struct Vertex {
    key: i32,
    distance: f64,
}

/// Struct for the priority queue.
/// 
/// Uses a min-heap for O(logV) removal of highest priority element.
/// Keeps an auxillary lookup table with a HashMap representation for
/// O(logV) relaxation of edges.
/// 
/// The min heap is ordered by giving highest priority to the vertex with
/// the shortest distance. 
pub struct PriorityQueue {
    heap: Vec<Vertex>,
    lookup_table: HashMap<i32, usize>,
}

// orderings so you can use `vertex1 < vertex2` or `vertex1 == vertex2`
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Vertex { }

impl Vertex {
    /// Creates new Vertex with vertex key k and distance d.
    pub fn new_with_priority(k: i32, d: f64) -> Self {
        Self{
            key: k,
            distance: d,
        }
    }

    /// Returns the key of itself.
    pub fn get_key(&self) -> i32 {
        self.key
    }

    /// Returns the distance of itself.
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}

// the impl block allows for object-oriented functionalty in Rust
// allows us functionality like `Q.pop()`.
impl PriorityQueue {
    /// Creates a new PriorityQueue struct.
    pub fn new() -> Self {
        let vec: Vec<Vertex> = Vec::new();
        let hm: HashMap<i32, usize> = HashMap::new();
        Self{
            heap: vec,
            lookup_table: hm,
        }
    }

    /// Returns the length of the PriorityQueue's heap structure.
    /// Operates in O(1) time.
    pub fn length(&self) -> usize {
        self.heap.len()
    }

    /// Adds a Vertex to the PriorityQueue with the specified key and distance.
    /// Operates in O(logV) time as a result of the call to percolate_up().
    pub fn insert_with_priority(&mut self, vertex: i32, weight: f64) {
        let v = Vertex::new_with_priority(vertex, weight);
        self.heap.push(v);
        let index = self.heap.iter().position(|n| n.key == vertex);
        self.lookup_table.insert(vertex, index.unwrap());
        self.percolate_up(vertex);
    }

    /// Updates specified key with new distance and percolates up accordingly
    /// Operates in O(logV) as a result of the call to percolate_up().
    pub fn relax(&mut self, key: i32, new_distance: f64) {
        let to_percolate = self.lookup_table.get(&key).copied();
        if let Some(v) = to_percolate {
            self.heap[v].distance = new_distance;
            self.percolate_up(key);
        }
        else { return; }       
    } 

    /// Removes the closest vertex from the PriorityQueue
    /// and updates the heap/lookup table.
    /// Operates in O(logV) time.
    pub fn pop(&mut self) -> Option<Vertex> {
        // panic if heap is empty
        if !(self.length() > 0) {
            return None;
        }

        // swap with last element
        self.swap(0, self.length() - 1);

        let smallest = self.heap.pop().unwrap();
        let _option = self.lookup_table.remove(&smallest.key);

        if self.length() == 0 {
            return Some(smallest);
        } // otherwise there is reason to sift and it will not panic
        
        // sift down
        self.sift_down(0);

        Some(smallest)
    }

    /// Function to check if the heap contains the specified key.
    /// Operates in O(1) time.
    pub fn contains_key(&self, key: i32) -> bool {
        self.lookup_table.contains_key(&key)
    }

    /// Swaps indices in heap and updates lookup table
    /// Operates in O(1) time.
    fn swap(&mut self, index1: usize, index2: usize) {
        self.heap.swap(index1, index2);
        self.lookup_table.insert(self.heap[index1].key, index1);
        self.lookup_table.insert(self.heap[index2].key, index2);
    }

    /// Percolates up at the index with the specified key.
    /// 
    /// Note that the lookup table is crucial in this operation.
    /// Operates in O(logV) time.
    fn percolate_up(&mut self, key: i32) {
        let to_percolate = self.lookup_table.get(&key).copied().unwrap();
    
        let mut index = to_percolate;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[index].distance < self.heap[parent].distance {
                self.swap(index, parent);
            }
            index = parent;
        }
    }

    /// Sifts down at the index with the specified key.
    /// Operates in O(logV) time.
    fn sift_down(&mut self, mut index: usize) {
        while ((index * 2) + 1) <= self.length() - 1 {
            let left = (2 * index) + 1;
            let right = (2 * index) + 2;
            let mut smaller = index;
            if left < self.length() && self.heap[left].distance < self.heap[smaller].distance {
                smaller = left;
            }
            if right < self.length() && self.heap[right].distance < self.heap[smaller].distance {
                smaller = right;
            }
            if smaller != index {
                self.swap(index, smaller);
                index = smaller;
            }
            else { return; }
        }
    }
}
