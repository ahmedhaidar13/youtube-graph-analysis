mod graph;

use graph::load_graph;

fn main() -> std::io::Result<()> {
    let file_path = "com-youtube.ungraph.txt";

    // Load the graph
    println!("Loading the graph...");
    let _graph = load_graph(file_path);
    println!();

    Ok(())
}
