mod graph;

use graph::{calculate_degree_centrality, find_connected_components, load_graph};

fn main() -> std::io::Result<()> {
    let file_path = "com-youtube.ungraph.txt";

    //Loading the graph
    println!("Loading the graph...");
    let graph = load_graph(file_path);
    println!();

    // calculating degree centrality
    calculate_degree_centrality(&graph);

    //finding and analyze connected components
    find_connected_components(&graph);

    Ok(())
}
