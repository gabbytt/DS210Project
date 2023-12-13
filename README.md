The statistics in this directory are from the following dataset:
https://snap.stanford.edu/data/amazon0505.html

## What My Project Does

Here's an overview of the main functionalities:

- `Initializes a new `AmazonNetwork` struct with an empty graph and an empty HashMap to store node indices.
- `read_data`: Reads data from a CSV file and constructs the graph based on the provided node connections.
- `nodes_with_extreme_degrees` and `nodes_with_most_and_least_connections`: Find nodes with the highest and lowest degrees in the graph.
- `calculate_degree_distribution`: Calculates the distribution of node degrees in the graph.
- `calculate_average_degree`: Computes the average degree of the nodes in the graph.
- `calculate_degree_centrality`: Calculates the degree centrality for each node in the graph.
- `visualize_graph`: Generates a DOT representation of the graph and writes it to a file named "graph.dot".
- `compute_centralities`: Computes and prints out the degree centrality for each node.

In the `main` function, it reads data from a CSV file, constructs the graph, calculates various graph statistics, computes centrality measures, and prints out the results.


## How To Run It

## What The Output Looks Like
