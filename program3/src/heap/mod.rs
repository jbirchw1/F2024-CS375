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

fn percolate_down(index: usize, mut heap: Vec<i32>) -> Vec<i32> {
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    let mut curr_smallest = index;

    if left < heap.len() && right < heap.len() {
        if heap[left] < heap[right] && heap[left] < heap[curr_smallest] {
            curr_smallest = left;
        }
        else if heap[left] > heap[right] && heap[right] < heap[curr_smallest] {
            curr_smallest = right;
        }
    }
    else if left < heap.len() {
        if heap[left] < heap[curr_smallest] {
            curr_smallest = left;
        }
    }
    // check to see if current index needs to be percolated downwards at all
    if curr_smallest != index {
        let temp : i32 = heap[index];
        heap[index] = heap[curr_smallest];
        heap[curr_smallest] = temp;
        // recurse
        heap = percolate_down(curr_smallest, heap);
    }
    // return
    heap
}

fn extract_min(mut heap: Vec<i32>) -> (i32, Vec<i32>) {
    // Remove from root of heap, replace with last element in vector, and percolate down
    let minimum : i32 = heap[0];
    let last_index = heap.len() - 1;
    let last_element = heap[last_index];
    heap[0] = last_element;
    heap.pop();
    if heap.len() == 0 {
        return (minimum, heap);
    }
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

    #[test]
    fn test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
