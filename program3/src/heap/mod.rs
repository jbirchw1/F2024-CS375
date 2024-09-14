
fn percolate_up() {}

fn percolate_down(index: usize, mut heap: Vec<i32>) -> Vec<i32> {
    while ((index * 2 + 1) < heap.len()) ((index * 2 + 2) < heap.len() - 1) {
        let left : usize = index * 2 + 1;
        let right : usize = index * 2 + 2;
        if heap[left] < heap[index] { // swap left child
            let temp : i32 = heap[left];
            heap[left] = heap[index];
            heap[index] = temp;
        }
        else if heap[right] < heap[index] {
            let temp : i32 = heap[right];
            heap[right] = heap[index];
            heap[index] = temp;
        }
        else {
            return heap;
        }
    }
    heap
}

fn extract_min(mut heap: Vec<i32>) -> (i32, Vec<i32>) {
    // TODO: extract minimum
    // Remove from root of heap, replace with last element in vector, and percolate down
    let minimum : i32 = heap[0];
    let last_index = heap.len() - 1;
    let last_element = heap[last_index];
    heap[0] = last_element;
    heap.pop();
    let resized_heap = percolate_down(0, heap);
    (minimum, resized_heap)
}

fn heapify() {
    // TODO: Recursive building of heap called from build_min_heap
}

//  O(n)
pub fn build_min_heap(mut unsorted_heap: Vec<i32>) -> Vec<i32> {
    // index of last non-leaf node == (len / 2) - 1
    // TODO: Call heapify() instead, right now causing errors
    let mut index : i32 = ((unsorted_heap.len() / 2) - 1).try_into().unwrap();
    while index >= 0 {
        unsorted_heap = percolate_down(index.try_into().unwrap(), unsorted_heap);
        index -= 1;
    }
    unsorted_heap
}

pub fn sort(unsorted_array: Vec<i32>) -> Vec<i32> {
    // TODO: Sort
    let mut sorted_list: Vec<i32> = Vec::new();
    let mut min_heap = build_min_heap(unsorted_array);
    while min_heap.len() > 0 {
        let smallest : i32;
        (smallest, min_heap) = extract_min(min_heap);
        sorted_list.push(smallest);
    }
    println!("HERE");
    println!("{:?}", sorted_list);
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
