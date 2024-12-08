use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// structuring graph using an adjacency list
pub struct Graph {
    pub adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    ///creates a new empty graph
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

    ///total number of nodes
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    /// total number of edges
    pub fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>()
            / 2 // Each edge is counted twice
    }
}

/// start by loading the graph from a file
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

/// calculate and display the degree centrality of the graph
pub fn calculate_degree_centrality(graph: &Graph) {
    println!("Calculating degree centrality...");
    let mut degrees: Vec<(u32, usize)> = graph
        .adjacency_list
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.len()))
        .collect();

    degrees.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top 10 nodes by degree centrality:");
    for (i, (node, degree)) in degrees.iter().take(10).enumerate() {
        println!("Rank {}: Node {} with degree {}", i + 1, node, degree);
    }
    println!();
}

///a function which detects and analyzes connected components in the graph
pub fn find_connected_components(graph: &Graph) {
    println!("Analyzing connected components...");
    println!();

    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &node in graph.adjacency_list.keys() {
        if !visited.contains(&node) {
            let mut component = HashSet::new();
            let mut stack = vec![node];
            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    component.insert(current);
                    if let Some(neighbors) = graph.adjacency_list.get(&current) {
                        stack.extend(neighbors);
                    }
                }
            }
            components.push(component);
        }
    }

    println!("Number of connected components: {}", components.len());
    if let Some(largest) = components.iter().max_by_key(|c| c.len()) {
        println!("Largest connected component size: {}", largest.len());
    }
    println!();
}
