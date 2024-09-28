mod binary_search_tree;

fn collect_input() -> (usize, usize, Vec<i32>) {
    // In rust, command line arguments can be collected from std::env,
    // while input provided via file redirection is collected from stdin

    // skip first argument, and collect the optional indeces
    let args: Vec<String> = env::args().skip(1).collect();

    // Set to 0 and 1 for initialization purposes.
    // In practice, these will virtually always be set correctly later in the program.
    let start: usize;
    let end: usize;

    // Read integers from stdin
    // Important: must read in size of unsorted array here in order to return appropriate
    // ending index in the event there are no command line arguments provided
    let stdin = io::stdin();
    let unsorted_array: Vec<i32> = stdin
        .lock()
        .lines()
        .skip(1)
        .filter_map(|line| line.ok()) // filter out any lines that couldn't be read
        .filter_map(|line| line.trim().parse().ok()) // convert each line to an i32
        .collect();

    // handles errors in command line input and appropriately assigns start and end indices for printing
    if args.len() == 1 {
        println!("ID-10-t Error: Not enough indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
        std::process::exit(0);
    } else if args.len() == 2 {
        // case in which the user wishes to output only the items between two vertices
        start = match args[0].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0); // Force quit program. Will not behave as expected if it continues
            }
        };
        end = match args[1].parse::<usize>() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                std::process::exit(0);
            }
        };

        // User error checking
        if start > unsorted_array.len() {
            println!("ID-10-t Error: starting index {} out of bounds.", start);
            std::process::exit(0);
        } else if end > unsorted_array.len() {
            println!("ID-10-t Error: ending index {} out of bounds.", end);
            std::process::exit(0);
        }

        return (start, end, unsorted_array);
    } else if args.len() > 2 {
        println!("ID-10-t Error: Too many indices entered. Provide either 0 (prints entire array) or 2 [start, end).");
        std::process::exit(0);
    }
    // case in which no input was provided => print entire array
    start = 0;
    end = unsorted_array.len();

    (start, end, unsorted_array)
}

fn parse_input(raw_in: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    
}

fn main() {
    println!("Hello, world!");
    let vec1 = Vec::from([4, 8, 1, 5, 9]);
    let mut my_bst = binary_search_tree::BinarySearchTree::from_vec(vec1);
    println!("Here is the first tree in order:");
    my_bst.in_order_traversal();
    my_bst.delete(5);
    println!("After deleting 5:");
    my_bst.in_order_traversal();
}
