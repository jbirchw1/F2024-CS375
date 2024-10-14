mod bfs;

fn main() {
    let mut graph = bfs::build_graph_from_stdin();
    bfs::print_graph(&graph);
}
