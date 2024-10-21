// module for the priority queue
use std::collections::{HashMap};
use std::cmp::Ordering;

pub struct Vertex {
    key: i32,
    distance: f64,
}

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

    pub fn get_key(&self) -> i32 {
        self.key
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}

// TODO: Reorder functions (once complete) by pub/priv
// the impl block allows for object-oriented functionalty in Rust
// allows us functionality like `Q.pop()`.
impl PriorityQueue {
    pub fn new() -> Self {
        let vec: Vec<Vertex> = Vec::new();
        let hm: HashMap<i32, usize> = HashMap::new();
        Self{
            heap: vec,
            lookup_table: hm,
        }
    }

    // returns length of pq
    pub fn length(&self) -> usize {
        self.heap.len()
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

        println!("Exited IWP");
    }

    // Updates specified key with new distance and percolates up accordingly
    pub fn relax(&mut self, key: i32, new_distance: f64) {
        let to_percolate = self.lookup_table.get(&key).copied();
        if let Some(v) = to_percolate {
            self.heap[v].distance = new_distance;
            self.percolate_up(key);
        }
        else { return; }       
    } 

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

        for (key, value) in &self.lookup_table {
            println!("{}: {}", key, value);
        }

        Some(smallest)
    }

    // Swaps indices in heap and updates lookup table
    fn swap(&mut self, index1: usize, index2: usize) {
        self.heap.swap(index1, index2);
        self.lookup_table.insert(self.heap[index1].key, index1);
        self.lookup_table.insert(self.heap[index2].key, index2);
    }

    // percolates up at the index with the specified key
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
            if self.heap[index].distance < self.heap[parent].distance {
                println!("swapping");
                self.swap(index, parent);
            }
            println!("value at the same index {} is now {}", index, self.heap[index].key);
            index = parent;
        }

        for (key, value) in &self.lookup_table {
            println!("{}: {}", key, value);
        }

        println!("Exited percolate up");
    }

    fn sift_down(&mut self, mut index: usize) {
        let to_sift = self.heap[index].key;
        println!("value to sift = {} at index {}", to_sift, index);
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
