// TODO: Make testable

// TODO: heap sort
//  - will need an insert function, which should call
//    percolate up. Will also need to call insert from
//    build max heap function. Will also need to call
//    a pecolate down function from extract max function.


pub fn sort(unsorted_array: Vec<i32>) {
    // println!("Need to sort");
}

pub fn build() {
    // println!("Need to build");
}

// Tests
// ? Could use some better docs ?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}