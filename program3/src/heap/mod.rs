fn print_heap_tree(heap: Vec<i32>, index: usize, prefix: String, is_left: bool) {
    if index >= heap.len() {
        return;
    }

    // Print the current node with the correct branch ("├──" or "└──")
    if index == 0 {
        println!(
            "{}\x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix, heap[index]
        );
    } else if (2 * index + 2) > heap.len() {
        println!(
            "{}{}\x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix,
            if is_left { "└── " } else { "├── " },
            heap[index]
        );
    } else {
        println!(
            "{}├── \x1b[38;2;0;0;255m{}\x1b[38;2;255;255;255m",
            prefix, heap[index]
        );
    }

    // Prepare prefix for child branches
    let mut child_prefix = format!("{}│   ", prefix);
    if index == 0 {
        child_prefix = String::new();
    }

    // Recursively print left and right children
    let left_child = 2 * index + 1;
    let right_child = 2 * index + 2;

    if right_child < heap.len() {
        print_heap_tree(heap.clone(), right_child, child_prefix.clone(), false);
    }
    if left_child < heap.len() {
        print_heap_tree(heap, left_child, child_prefix, true);
    }
}

fn percolate_down(mut index: usize, mut heap: Vec<i32>) -> Vec<i32> {
    let length = heap.len();

    while 2 * index + 1 < length {
        // While left child exists
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut curr_smallest = index;

        if left < length && heap[left] < heap[curr_smallest] {
            curr_smallest = left;
        }
        if right < length && heap[right] < heap[curr_smallest] {
            curr_smallest = right;
        }

        if curr_smallest == index {
            return heap;
        }

        heap.swap(index, curr_smallest);
        index = curr_smallest;
    }

    heap
}

fn extract_min(mut heap: Vec<i32>) -> (i32, Vec<i32>) {
    if heap.is_empty() {
        panic!("Error ID-10-t: cannot extract minimum from empty heap");
    }

    let minimum = heap[0];

    if heap.len() == 1 {
        heap.pop();
        return (minimum, heap);
    }

    let last_index = heap.len() - 1;
    let last_element = heap[last_index];
    heap[0] = last_element;
    heap.pop();

    let resized_heap = percolate_down(0, heap);

    (minimum, resized_heap)
}

fn heapify(mut heap: Vec<i32>) -> Vec<i32> {
    let mut index: i32 = (heap.len() / 2 - 1).try_into().unwrap();
    while index >= 0 {
        heap = percolate_down(index.try_into().unwrap(), heap);
        index -= 1;
    }
    print_heap_tree(heap.clone(), 0, String::new(), false);
    heap
}

pub fn build_min_heap(unsorted_heap: Vec<i32>) -> Vec<i32> {
    heapify(unsorted_heap)
}

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

// Tests
// ? Could use some better docs ?
#[cfg(test)]
mod tests {
    use super::*;
    use crate::gendata::{generate_max_random_list, generate_random_list};

    // test to make sure tests work
    #[test]
    fn test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

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
        // println!("{:?}", sorted_array);
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

    // less of a unit test, but a useful test nevertheless
    #[test]
    fn assert_sorted() {
        let unsorted_array = generate_max_random_list(1000000);
        let mut unsorted_vector = unsorted_array.clone();
        let heapsorted_vector = sort(unsorted_array);
        unsorted_vector.sort();
        assert_eq!(unsorted_vector, heapsorted_vector);
    }
}
