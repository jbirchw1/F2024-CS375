# Breadth First Search
Here is my breadth-first search implementation in Rust. For instructions on how to use and compile, please take a look at program 2 [here](https://github.com/jbirchw1/F2024-CS375/tree/main/program2). 

## Input 
Please provide all input in the form specified by the project instructions. That is, where the first line contains the number of vertices & edges, and each subsequent line is a directed edge. Please provide all vertex keys in integer format. In the future, however, I plan to expand the program to be able to handle vertex keys of abstract type.

Please pass the root and destination vetices on the command line as arguments and pass the input file from stdin/file redirection. i.e., the command line should look something like

`./program5 0 3 < input.txt`,

where 0 is the root, 3 is the destination, and `input.txt` contains the graph edges. Alternatively, you can pass it all from the cargo run like so:

`cargo run -- 0 3 < input.txt`.

However, this may have the unintended side effect of printing the output from `cargo build` to the terminal, so to avoid extra output, use `cargo build` first and then execute it as a typical program. 

If you have any other questions about the code or its execution, please email me at jbirchw1@binghamton.edu. 
