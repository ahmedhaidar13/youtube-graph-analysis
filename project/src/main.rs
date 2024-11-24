mod graph;

use graph::{load_graph, calculate_degree_centrality, find_connected_components};

fn main() -> std::io::Result<()> {
    let file_path = "com-youtube.ungraph.txt";

    //loading the graph
    let graph = load_graph(file_path);

    //calculate degree centrality
    calculate_degree_centrality(&graph);

    // find and analyze connected components
    find_connected_components(&graph);

    Ok(())
}
