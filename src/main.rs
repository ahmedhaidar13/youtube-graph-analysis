mod graph;

use graph::{calculate_degree_centrality, find_connected_components, load_graph, Graph};
use std::collections::{HashMap, VecDeque};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() -> std::io::Result<()> {
    let file_path = "com-youtube.ungraph.txt";

    //Loading the graph
    println!("Loading the graph...");
    let graph = load_graph(file_path);
    println!();

    // implementing all functions in main
    // calculating degree centrality
    calculate_degree_centrality(&graph);

    //finding and analyzing connected components
    find_connected_components(&graph);

    // Analyze connectivity
    analyze_connectivity_bfs(&graph);

    Ok(())
}

///new function that analyzes connectivity and computes the average shortest path length using bfs
fn analyze_connectivity_bfs(graph: &Graph) {
    println!("Analyzing connectivity...");
    println!();

    println!("Computing average shortest path length using BFS...");
    let nodes: Vec<u32> = graph.adjacency_list.keys().cloned().collect();
    let sample_size = 20;
    let mut rng = thread_rng();
    let sample_nodes = nodes.choose_multiple(&mut rng, sample_size).cloned();

    let mut total_path_length = 0.0;
    let mut count = 0;

    for source in sample_nodes {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        distances.insert(source, 0);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            if let Some(neighbors) = graph.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, current_distance + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        for &distance in distances.values() {
            total_path_length += distance as f64;
            count += 1;
        }
    }

    let avg_path_length = total_path_length / count as f64;
    println!(
        "Estimated average shortest path length for the graph: {:.4}",
        avg_path_length
    );
    println!();
}

// all tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, VecDeque};

    // verifying that adding edges correctly updates node and edge count in graph
    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    //confirms degree centrality is calculated correctly and identifies top node
    #[test]
    fn test_degree_centrality() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);

        let mut degrees: Vec<(u32, usize)> = graph
            .adjacency_list
            .iter()
            .map(|(node, neighbors)| (*node, neighbors.len()))
            .collect();

        degrees.sort_by(|a, b| b.1.cmp(&a.1)); //sorting by degree in descending order

        assert_eq!(degrees[0], (1, 3)); 
        assert_eq!(degrees[1].1, 1);
        assert_eq!(degrees.len(), 4); 
    }

    // verifies that the connected components are identified correctly
    #[test]
    fn test_connected_components() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let mut visited = std::collections::HashSet::new();
        let mut components = Vec::new();

        for &node in graph.adjacency_list.keys() {
            if !visited.contains(&node) {
                let mut component = std::collections::HashSet::new();
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

        assert_eq!(components.len(), 2); //Two connected components
    }

    //confirms that bfs computes the shortest path distance correctly
    #[test]
    fn test_bfs_shortest_path() {
        //creating a small graph
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        //expected distances from node 1
        let expected_distances: HashMap<u32, u32> = vec![
            (1, 0), 
            (2, 1), 
            (3, 1), 
            (4, 2), 
        ]
        .into_iter()
        .collect();

        //Performing BFS starting from node 1
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        distances.insert(1, 0);
        queue.push_back(1);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            if let Some(neighbors) = graph.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, current_distance + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        //asserting that BFS computed the correct distances
        assert_eq!(distances, expected_distances);
    }
}