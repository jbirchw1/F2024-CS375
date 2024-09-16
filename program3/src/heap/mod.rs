// mod gendata;

// under construction
// fn print_binary_tree(array : &Vec<i32>) {
//     let p2 = array.len().next_power_of_two();
//     let height = (p2 as f32).log2().ceil() as i32;
//     let mut curr : usize = 0;
//     let base : u32 = 2;
//     let max_length : i32 = height * 2 + 1;
//     for i in 1..height+1 {
//         let length : i32 = max_length / (2 * i);
//         let empty_spaces = (0..length).map(|_| "-").collect::<String>();
//         if i % 2 == 1 {
//             let j = 0;
//             while (j < base.pow(i.try_into().unwrap())) && (curr < array.len()) {
//                 print!("{}{}", empty_spaces, array[curr]);
//                 curr += 1;
//             }
//             println!("{}", empty_spaces);
//         }
//         else {
//             let j = 0;
//             while (j < base.pow(i.try_into().unwrap())) && (curr < array.len()) {
//                 print!("{}{}", array[curr], empty_spaces);
//                 curr += 1;
//             }
//             println!("{}", array[curr]);
//             curr += 1;
//         }
//     }
// }

fn percolate_down(mut index: usize, mut heap: Vec<i32>) -> Vec<i32> {
    let length = heap.len();

    while 2 * index + 1 < length { // While left child exists
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
        panic!("cannot extract minimum from empty heap");
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

fn heapify(mut heap : Vec<i32>) -> Vec<i32> {
    let mut index : i32 = (heap.len() / 2 - 1).try_into().unwrap();
    while index >= 0 {
        heap = percolate_down(index.try_into().unwrap(), heap);
        index -= 1;
    }
    heap
}

pub fn build_min_heap(unsorted_heap: Vec<i32>) -> Vec<i32> {
    heapify(unsorted_heap)
}

pub fn sort(unsorted_array: Vec<i32>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = Vec::new();
    let mut min_heap = build_min_heap(unsorted_array);
    while min_heap.len() > 0 {
        let smallest : i32;
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
        for i in 0..(sorted_array.len()/2-1) {
            if (i as usize) * 2 + 1 < sorted_array.len() {
                let left = sorted_array[(i as usize) * 2 + 1];
                if !(sorted_array[i as usize] <= left) {
                    panic!("{} at index {} is not less than child {} at index {}", sorted_array[i as usize], i, left, 2*i+1);
                }
            }
            if (i as usize) * 2 + 2 < sorted_array.len() {
                let right = sorted_array[(i as usize) * 2 + 2];
                if !(sorted_array[i as usize] <= right) {
                    panic!("{} is not less than child {}", sorted_array[i as usize], right);
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
                    panic!("{} is not less than alleged minimum {}", remaining_heap[i], alleged_minimum);
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
