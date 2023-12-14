use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

struct AmazonNetwork {
    graph: DiGraph<String, ()>,
    node_indices: HashMap<usize, NodeIndex>,
}

impl AmazonNetwork {
    fn new() -> Self {
        AmazonNetwork {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    fn nodes_with_extreme_degrees(&self) -> Option<(NodeIndex, NodeIndex)> {
        // Identifies nodes with the highest and lowest degrees in the graph
        let (node_max, _) = self.graph.node_indices().map(|node| {
            let degree = self.graph.edges(node).count();
            (node, degree)
        }).max_by_key(|&(_, degree)| degree)?;

        let (node_min, _) = self.graph.node_indices().map(|node| {
            let degree = self.graph.edges(node).count();
            (node, degree)
        }).min_by_key(|&(_, degree)| degree)?;

        Some((node_max, node_min))
    }

    fn nodes_with_most_and_least_connections(&self) -> Option<(NodeIndex, NodeIndex)> {
        // Finds nodes with the most and least connections
        let (node_most, _) = self.graph.node_indices().map(|node| {
            let degree = self.graph.edges(node).count();
            (node, degree)
        }).max_by_key(|&(_, degree)| degree)?;

        let (node_least, _) = self.graph.node_indices().map(|node| {
            let degree = self.graph.edges(node).count();
            (node, degree)
        }).min_by_key(|&(_, degree)| degree)?;

        Some((node_most, node_least))
    }

    fn read_data(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(file);

        for result in reader.records() {
            let record = result?;
            let from_node_id: usize = record[0].parse()?;
            let to_node_id: usize = record[1].parse()?;

            let from_label = format!("Node {}", from_node_id);
            let to_label = format!("Node {}", to_node_id);

            let from_node = *self.node_indices
                .entry(from_node_id)
                .or_insert_with(|| {
                    let node = self.graph.add_node(from_label.clone());
                    node
                });
            let to_node = *self.node_indices
                .entry(to_node_id)
                .or_insert_with(|| {
                    let node = self.graph.add_node(to_label.clone());
                    node
                });

            self.graph.add_edge(from_node, to_node, ());
        }

        Ok(())
    }

    fn calculate_degree_distribution(&self) -> HashMap<usize, usize> {
        // Calculates the degree distribution of the nodes
        let mut degree_distribution = HashMap::new();
    
        for node in self.graph.node_indices() {
            let degree = self.graph.edges(node).count();
            *degree_distribution.entry(degree).or_insert(0) += 1;
        }
    
        degree_distribution
    }
    
    fn calculate_average_degree(&self) -> f64 {
        // Computes the average degree of the nodes in graph
        let total_degree: usize = self.graph.node_indices()
            .map(|node| self.graph.edges(node).count())
            .sum();
        let node_count = self.graph.node_count();
    
        if node_count > 0 {
            total_degree as f64 / node_count as f64
        } else {
            0.0
        }
    }
    
    fn calculate_degree_centrality(&self) -> HashMap<NodeIndex, f64> {
        // Calculates the degree centrality for each node
        let mut degree_centrality = HashMap::new();

        for node in self.graph.node_indices() {
            let degree = self.graph.edges(node).count() as f64;
            let centrality = degree / (self.graph.node_count() - 1) as f64;
            degree_centrality.insert(node, centrality);
        }

        degree_centrality
    }
    

    fn visualize_graph(&self) {
        // Generates a DOT file to visualize graph
        let dot = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        std::fs::write("graph.dot", dot).expect("Unable to write DOT file");
    }


    fn compute_centralities(&self) -> Result<(), Box<dyn Error>> {
        // Computes and prints degree centrality for nodes
        let degree_centrality = self.calculate_degree_centrality();
        println!("Degree Centrality: {:?}", degree_centrality);

        Ok(())
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "amazondata.csv";

    let mut amazon_network = AmazonNetwork::new();
    amazon_network.read_data(file_path)?;

    amazon_network.compute_centralities()?;

    let degree_distribution = amazon_network.calculate_degree_distribution();
    let average_degree = amazon_network.calculate_average_degree();
    amazon_network.visualize_graph();

    println!("Degree Distribution: {:?}", degree_distribution);
    println!("Average Degree: {}", average_degree);

    let nodes_extreme_degrees = amazon_network.nodes_with_extreme_degrees();
    if let Some((node_max_degree, node_min_degree)) = nodes_extreme_degrees {
        println!("Node with the highest degree: {:?}", node_max_degree);
        println!("Node with the lowest degree: {:?}", node_min_degree);
    } else {
        println!("No nodes found in the graph.");
    }

    let nodes_most_least_connections = amazon_network.nodes_with_most_and_least_connections();
    if let Some((node_most_connections, node_least_connections)) = nodes_most_least_connections {
        println!("Node with the most connections: {:?}", node_most_connections);
        println!("Node with the least connections: {:?}", node_least_connections);
    } else {
        println!("No nodes found in the graph.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_with_extreme_degrees() {
        // Create an instance of AmazonNetwork
        // a test case for the nodes_with_extreme_degrees method to ensure it correctly identifies nodes with extreme degrees
        let mut amazon_network = AmazonNetwork::new();

        let nodes = vec![
            (1, 2),
            (1, 3),
            (2, 3),
        ];

        for &(from, to) in nodes.iter() {
            let from_node = *amazon_network.node_indices
                .entry(from)
                .or_insert_with(|| {
                    let node = amazon_network.graph.add_node(format!("Node {}", from));
                    node
                });

            let to_node = *amazon_network.node_indices
                .entry(to)
                .or_insert_with(|| {
                    let node = amazon_network.graph.add_node(format!("Node {}", to));
                    node
                });

            amazon_network.graph.add_edge(from_node, to_node, ());
        }

        let result = amazon_network.nodes_with_extreme_degrees();

        let expected_result = Some((NodeIndex::new(0), NodeIndex::new(2)));

        assert_eq!(result, expected_result);
    }
}

