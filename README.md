The statistics in this directory are from the following dataset:
https://snap.stanford.edu/data/amazon0505.html

## What My Project Does

Here's an overview of the main functionalities:

- `Initializes a new `AmazonNetwork` struct with an empty graph and an empty HashMap to store nodes.
- `read_data`: Reads data from a CSV file and constructs the graph based on the provided node connections.
- `nodes_with_extreme_degrees` and `nodes_with_most_and_least_connections`: Find nodes with the highest and lowest degrees in the graph.
- `calculate_degree_distribution`: Calculates the distribution of node degrees in the graph.
- `calculate_average_degree`: Computes the average degree of the nodes in the graph.
- `calculate_degree_centrality`: Calculates the degree centrality for each node in the graph.
- `visualize_graph`: Generates a DOT representation of the graph and writes it to a file named "graph.dot".
- `compute_centralities`: Computes and prints out the degree centrality for each node.

Overall, the main function reads data from a CSV file, constructs the graph, calculates various graph statistics, computes centrality measures, and prints out the results.


## How To Run It
Ensure you have a data file named amazondata.csv in the same directory as the code.
Once executing the code with "Cargo run":

It will read the amazondata.csv file, perform computations on the network data, and display various metrics and information about the Amazon network.

There is also a test that can be run. This test is used to verify the correctness of the nodes_with_extreme_degrees method by checking if it correctly identifies nodes with extreme degrees in a specific graph scenario defined in the test case. 

## What The Output Looks Like
The output should display metrics like degree distribution, average degree, nodes with extreme degrees, nodes with most/least connections, and degree centrality. 

You'll see in the output for this particular dataset that it shows:
- 76,515 nodes in the network have a degree of 0.
- There are 96,508 nodes with a degree of 10.
- The rest of the nodes have varying degrees between 1 and 9.
- An average degree of approximately 5.57
- The centrality values were either 0 or 5.312762317639433e-5.

This distribution gives insight into how many nodes in the network are connected to others and the overall structure of the co-purchasing relationships among Amazon products.
The degree of a product node would show how many other products it is frequently co-purchased with. Nodes with higher degrees might indicate products often bought together, forming a more interconnected group within the network. 

On average, each node in the graph is connected to around 5.57 other nodes. This average degree suggests that, on average, each product (node) within the network is associated or co-purchased with roughly 5.57 other products.

 When computing centralities for the nodes in the network, the resulting centrality values tended to fall into two categories: either 0 or 5.312762317639433e-5. This suggests that the computed centralities were either negligible (close to zero) or had this non-zero value.


