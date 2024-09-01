## Programming Assignment 1 - Binary Search Algorithm ##
### Instructions for use ###
To compile, you have 2 options:
1. Navigate into the `src` directory and execute `rustc main.rs`, then execute like a normal executable.
 The program is designed to accept input via file redirection. In other words, you should execute as `./program1 < [input_file]`
2. From the top of the `program1` directory, execute `cargo run < [input_file]`. This will
 compile the code, install any needed dependencies and run the code using the input from the specified file.
 I suggest using this second method.
### Note ###
This program will not work if you try and execute with something like `./program1 4 1 2 3 4 1 3`. You MUST use file redirection and/or
piping an input file in. This is due to the difference between Rust and C/C++ in argument collection. A rust program will not treat
a seperate line as a different argument, instead stdin terminates at the newline. 
