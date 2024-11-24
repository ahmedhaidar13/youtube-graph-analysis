use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type GraphType = UnGraph<u32, ()>;

/// Load the graph from a file
pub fn load_graph(file_path: &str) -> GraphType {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph = UnGraph::<u32, ()>::new_undirected();

    // Track existing nodes
    let mut node_map: HashMap<u32, NodeIndex> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split('\t').collect();
        if nodes.len() == 2 {
            let node1: u32 = nodes[0].parse().unwrap();
            let node2: u32 = nodes[1].parse().unwrap();

            // Add nodes only if they don't already exist
            let node1_index = *node_map.entry(node1).or_insert_with(|| graph.add_node(node1));
            let node2_index = *node_map.entry(node2).or_insert_with(|| graph.add_node(node2));

            // Add edge between the nodes
            graph.add_edge(node1_index, node2_index, ());
        }
    }

    println!("Graph loaded successfully!");
    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());

    graph
}

/// Calculate and display the degree centrality of the graph
pub fn calculate_degree_centrality(graph: &GraphType) {
    let mut degrees = Vec::new();

    // Calculate degree for each node
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        degrees.push((node, degree));
    }

    // Sort nodes by degree in descending order
    degrees.sort_by(|a, b| b.1.cmp(&a.1));

    // Print the top 10 nodes with the highest degree
    println!("Top 10 nodes by degree centrality:");
    for (i, (node, degree)) in degrees.iter().take(10).enumerate() {
        println!(
            "Rank {}: Node {:?} with degree {}",
            i + 1,
            graph.node_weight(*node).unwrap(), // Access the node value correctly
            degree
        );
    }
}
