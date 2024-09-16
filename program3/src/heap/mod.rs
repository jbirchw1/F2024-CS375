// Public function to simplify interface.
#[allow(dead_code)] // turned on since the function shouldn't be called by default.
pub fn print_heap(heap: Vec<i32>) {
    print_heap_value(heap, 0, String::new(), false);
}

/**
 * function that takes an array-based heap of i32 values and prints to
 * the terminal in a file-tree type of format. Numbers are printed
 * in blue for clarity.
 * Takes as parameters an i32 vector, an index as usize which to print,
 * a string containing the prefix to print before the value, and a boolean
 * value determining if the value to be printed is a left or right child.
*/
#[allow(dead_code)] // dead code warnings are transitive, so to speak
                    // so since this function is only called in a "dead"
                    // function, it will also throw a warning unless
                    // explicitly ignored.
fn print_heap_value(heap: Vec<i32>, index: usize, prefix: String, is_left: bool) {
    // return if reached end of heap. I.e., base case.
    if index >= heap.len() {
        return;
    }

    // Print the current node (in blue) prefixed by formatted string
    if index == 0 {
        println!(
            "{}\x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix, heap[index]
        );
    // format leaf values differently, making it look nicer
    } else if (2 * index + 2) > heap.len() {
        println!(
            "{}{}\x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix,
            if is_left { "└── " } else { "├── " },
            heap[index]
        );
    // if not a leaf node, give the standard tree prefix
    } else {
        println!(
            "{}├── \x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix, heap[index]
        );
    }

    // prepare prefix for child branches
    let mut child_prefix = format!("{}│   ", prefix);
    if index == 0 {
        child_prefix = String::new();
    }

    // recursively print left and right children
    let left_child = 2 * index + 1;
    let right_child = 2 * index + 2;

    // call function recursively for both children
    // need to do heap.clone() for rust borrowing complacency. Still not 100% clear on this
    if right_child < heap.len() {
        print_heap_value(heap.clone(), right_child, child_prefix.clone(), false);
    }
    if left_child < heap.len() {
        print_heap_value(heap, left_child, child_prefix, true);
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
    // * NOTE: Uncomment the following line to print the formatted heap to the terminal.
    // print_heap(heap.clone());
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
// public function build_max_heap().
pub fn build_min_heap(unsorted_heap: Vec<i32>) -> Vec<i32> {
    heapify(unsorted_heap)
}

/**
 * Public function to sort an unsorted array and return a sorted array
 * using heaps. Simply builds a heap and extracts the minimum until the
 * heap is empty.
 * Takes the unsorted i32 vector as input
 * Returns the sorted i32 vector as output
 */
pub fn sort(unsorted_array: Vec<i32>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = Vec::new();
    let mut min_heap = build_min_heap(unsorted_array);
    while !min_heap.is_empty() {
        let smallest: i32;
        (smallest, min_heap) = extract_min(min_heap);
        sorted_list.push(smallest);
    }
    sorted_list
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*; // use private functions in this file
    use crate::gendata::{generate_max_random_list, generate_random_list}; // use other mod in the same crate

    // test to determine if values will percolate down correctly
    #[test]
    fn assert_percolating_down() {
        let unheap = Vec::from([10, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let heap = Vec::from([1, 3, 2, 7, 4, 5, 6, 10, 8, 9]);
        assert_eq!(heap, percolate_down(0, unheap));
    }

    // Test to determine if each left and right child is greater than its' parent
    #[test]
    fn assert_heap() {
        let unsorted_array = generate_random_list(1000000);
        let sorted_array = build_min_heap(unsorted_array);
        for i in 0..(sorted_array.len() / 2 - 1) {
            if (i as usize) * 2 + 1 < sorted_array.len() {
                let left = sorted_array[(i as usize) * 2 + 1];
                if !(sorted_array[i as usize] <= left) {
                    panic!(
                        "{} at index {} is not less than child {} at index {}",
                        sorted_array[i as usize],
                        i,
                        left,
                        2 * i + 1
                    );
                }
            }
            if (i as usize) * 2 + 2 < sorted_array.len() {
                let right = sorted_array[(i as usize) * 2 + 2];
                if !(sorted_array[i as usize] <= right) {
                    panic!(
                        "{} is not less than child {}",
                        sorted_array[i as usize], right
                    );
                }
            }
        }
    }

    // test to assert extracted minimum is in fact the minimum
    #[test]
    fn assert_minimum_extraction() {
        let mut sorted_array = build_min_heap(generate_max_random_list(10000));
        while sorted_array.len() > 0 {
            let (alleged_minimum, remaining_heap) = extract_min(sorted_array);
            for i in 0..remaining_heap.len() {
                if !(remaining_heap[i] > alleged_minimum) {
                    panic!(
                        "{} is not less than alleged minimum {}",
                        remaining_heap[i], alleged_minimum
                    );
                }
            }
            sorted_array = remaining_heap;
        }
    }

    // test to assert the sort is correct
    // less of a unit test, but a useful test nevertheless
    #[test]
    fn assert_sorted() {
        let unsorted_array = generate_max_random_list(1000000);
        let mut unsorted_vector = unsorted_array.clone();
        let heapsorted_vector = sort(unsorted_array);
        unsorted_vector.sort(); // known correct sorting algorithm
        assert_eq!(unsorted_vector, heapsorted_vector);
    }
}
