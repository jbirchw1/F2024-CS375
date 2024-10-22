# Breadth First Search
Here is my breadth-first search implementation in Rust. For instructions on how to use and compile, please take a look at program 2 [here](https://github.com/jbirchw1/F2024-CS375/tree/main/program2). 

## Input 
Please provide all input in the form specified by the project instructions. Each input file should be of the following form:

```
x y
u v d
```

where x, y are the number of vertices and edges in the graph, respectively. Each following line of the form u v d should specify an edge start (u), destination (v) and the weight (d).

Please pass the source and destination vetices on the command line as arguments and pass the input file from stdin/file redirection. i.e., the command line should look something like

`./program6 0 3 < input.txt`,

where 0 is the source, 3 is the destination, and `input.txt` contains the graph edges. Alternatively, you can pass it all from the cargo run like so:

`cargo run -- 0 3 < input.txt`.

However, this may have the unintended side effect of printing the output from `cargo build` to the terminal, so to avoid extra output, use `cargo build` first and then execute it as a typical program. 

## Documentation
I have taken advantage of the cargo doc feature to apply nicely formatted documentation to the code. To see this, simply execute

`cargo doc --open`

This will generate the documentation webpage and open it in your default web browser. If you asked me, I would say there is no reason to view my code in the terminal/editor when you can view both the source code and the (formatted) documentation on this page. This makes the examination of my project structure and each function's utility much easier. 

Otherwise, feel free to read the comments in your text editor, though the doc syntax *may* impact readability.

If you have any other questions about the code or its execution, please email me at jbirchw1@binghamton.edu. 
