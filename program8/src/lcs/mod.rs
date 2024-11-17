// module for the lcs functions

pub fn compute_lcs(first: &Vec<char>, second: &Vec<char>) -> Vec<Vec<char>> {
    let size_first = first.len();    // m
    let size_second = second.len();  // n

    // there's some weirdness happening here. Unclear why, but I found that skipping the first
    // element of the second string makes it work as expected.
    let mut values = vec![vec![0; size_second+1]; size_first];
    let mut directions = vec![vec![' '; size_second+1]; size_first];

    // set first row + column to 0
    for i in 0..size_first  { values[i][0] = 0; }
    for j in 0..size_second { values[0][j] = 0; }

    // compute table entries in row-major order
    for i in 1..size_first {
        for j in 0..size_second {
            // println!("Comparing values {} and {}", first[i], second[j]);
            if first[i-1] == second[j] { // match, element of lcs
                values[i][j+1] = values[i-1][j] + 1;
                directions[i][j+1] = '↖';
            }
            else if values[i-1][j+1] >= values[i][j] { // cell above is greater than cell left
                values[i][j+1] = values[i-1][j+1];
                directions[i][j+1] = '↑';
            }
            else { // cell left is greater than cell above
                values[i][j+1] = values[i][j];
                directions[i][j+1] = '←';
            }
        }
    }

    directions
}


pub fn print_output(directions: Vec<Vec<char>>, string: &Vec<char>, i: usize, j: usize) {
    let length = print_lcs(directions, &string, i, j, 0);
    println!(""); // new line
    println!("Length: {}", length);
}

// Helper function to recursively print lcs
fn print_lcs(directions: Vec<Vec<char>>, string: &Vec<char>, i: usize, j: usize, mut length: usize) -> usize {
    if i == 0 || j == 0 { 
        print!("LCS: ");
        return length; 
    } // LCS has remaining length 0
    if directions[i][j] == '↖' {
        length = print_lcs(directions, &string, i-1, j-1, length+1);
        print!("{}", string[i]);
        return length;
    }
    else if directions[i][j] == '↑' {
        length = print_lcs(directions, &string, i-1, j, length);
        return length;
    } 
    else {
        length = print_lcs(directions, &string, i, j-1, length);
        return length;
    }
}