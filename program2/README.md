# Merge Sort #

This is my merge sort implementation, coded mostly in Rust. The code itself contains enough comments to be self-explanatory, but if you have any confusion about the program's and/or the Rust compiler's use, please see below.
## Instructions For Use ##

Once the .tar.gz has been extracted, navigate to inside the program2 directory. From here, you have several options for compiling and running:
### rustc ###

This method requires you to know how to download, compile, and link crate dependencies from the command line. I do not know how. I highly suggest you not use this method, but if you must, I will take it for granted that you already know what you are doing.
### cargo build ###

If you want to execute the program more traditionally, e.g., `./program2`, you may use cargo to build first. From anywhere inside the `program2/` directory, execute `cargo build` and ensure the dependency (vecshard) has been downloaded and compiled. I have already ensured that this works on the CS department lab machines. Once built, navigate to the `target/debug` directory and look for the executable named `program2`. Then, you may execute this normally. It is crucial that the unsorted data is passed in the form of file redirection and optional indices are passed as command line arguments. My program gathers the input data from stdin and the command line arguments from `std::env`. The program will not behave as expected if input is mixed between these two sources. For example, execute `program2 [start] [end] < [input].txt`. 
### cargo run ###

Cargo provides an in-built method of building and running in one command while allowing command line input. From anywhere inside the `program2/` directory, execute `cargo build -- [start] [end] < [input].txt`. This will compile and run the program with the specified input file (and optional command line arguments). **Note**: if you do not wish to provide any start and end indices, then the `--` is unnecessary. Execute `cargo run < [input].txt` in this case. File redirection to an out file is also similar. E.g., `cargo run < [input].txt > output.txt`. 

## Provided Testing Script ##

If you are the grader, I imagine you already have your own test cases and whatnot. However, if you want to test it with random output, I have provided a script to do just that. To generate a random file of size n from the `program2/testing` directory, execute `./build_and_test.sh -l [n] -t`. This will generate a random, unsorted list and check the program's output against a known correct solution. For more information about the script and its capabilities, run `./build_and_test.sh -h` or `./build_and_test.sh --help`. 

If you have any other questions about the program's design, use or functionality, do not hesitate to contact me at jbirchw1@binghamton.edu. Happy sorting!
