# Heap Sort
This is my implementation of heap sort in Rust. The code should have ample comments, but for instructions on use and other fun things, please refer here.

## Instructions For Use
For instructions on how to compile and run the program, please refer to program2. In short, a typical `cargo run -- [start] [end] < [input].txt > [output].txt` will suffice. 

## Benchmarking
As part of the program's instructions, I have included several performance metrics comparing run times between merge sort (program 2) and heap sort. Using the `time` command, I measured each program's wall clock time of `cargo run` in seconds. Note that this program was run on the Binghamton University CS department's lab machines.

| Program    | Time for 1M Values     | Time for 10M Values      |
| ---------- | ---------------------- | ------------------------ |
| Merge Sort | 6.303s                 | 77.924s                  |
| Heap Sort  | 26.290s                | 301.117s                 |

Clearly, merge sort is faster than heap sort for large values. However, I also used Rust's Criterion crate for benchmark tests. This allows one to directly compare the average run times for each function given a specified input size. However, when executing `cargo bench`, we see that heap sort outperforms merge sort, up to 100k values. I encourage you to try this yourself; see the image below for a look at the output. I theorize that since the benchmark tests compare the sort functions directly and skip all the input parsing, we may see some improvements in the more space-efficient heap sort.

<img width="606" alt="Screenshot 2024-09-16 at 1 59 43 PM" src="https://github.com/user-attachments/assets/c28bdaff-425c-4135-a177-aefba1552f36">

## Testing:
I have also included a few unit tests for direct testing of each function, one for each of the following: percolating down; heapifying; extracting the minimum; sorting the array. If, for any reason, my program fails during evaluation, I ask you to run `cargo test` anywhere in the directory to determine where exactly it went wrong. Please email me at jbirchw1@binghamton.edu with any findings.
