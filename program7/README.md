# Prim's Algorithm
Here is my Prim's algorithm implementation in Rust. For instructions on how to use and compile, please take a look at program 2 [here](https://github.com/jbirchw1/F2024-CS375/tree/main/program2). 

## Input 
Please provide all input in the form specified by the project instructions. Each input file should be of the following form:

```
x y
u v d
```

where x, y are the number of vertices and edges in the graph, respectively. Each following line of the form u v d should specify an edge start (u), destination (v) and the weight (d).

Please pass the source and destination vetices on the command line as arguments and pass the input file from stdin/file redirection. i.e., the command line should look something like

`./prim 0 3 < input.txt`,

where 0 is the source, 3 is the destination, and `input.txt` contains the graph edges. Alternatively, you can pass it all from the cargo run like so:

`cargo run -- 0 3 < input.txt`.

However, this may have the unintended side effect of printing the output from `cargo build` to the terminal, so to avoid extra output, use `cargo build` first and then execute it as a typical program. 

## Documentation
Please take a look at program 6: Dijkstra's Algorithm [here](https://github.com/jbirchw1/F2024-CS375/tree/main/program6) for more information on
documentation and comments.