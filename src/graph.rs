use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Graph structure using an adjacency list
pub struct Graph {
    adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, node1: u32, node2: u32) {
        self.adjacency_list
            .entry(node1)
            .or_insert_with(HashSet::new)
            .insert(node2);
        self.adjacency_list
            .entry(node2)
            .or_insert_with(HashSet::new)
            .insert(node1);
    }

    /// Get the total number of nodes
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Get the total number of edges
    pub fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>()
            / 2 // Each edge is counted twice
    }
}

/// Load the graph from a file
pub fn load_graph(file_path: &str) -> Graph {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split('\t').collect();
        if nodes.len() == 2 {
            let node1 = nodes[0].parse::<u32>().unwrap();
            let node2 = nodes[1].parse::<u32>().unwrap();
            graph.add_edge(node1, node2);
        }
    }

    println!("Graph loaded successfully!");
    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());
    println!();

    graph
}
