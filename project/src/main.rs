mod graph;

use graph::{load_graph, calculate_degree_centrality};

fn main() -> std::io::Result<()> {
    // Path to the dataset
    let file_path = "com-youtube.ungraph.txt";

    // Load the graph
    let graph = load_graph(file_path);

    // Calculate degree centrality
    calculate_degree_centrality(&graph);

    Ok(())
}
