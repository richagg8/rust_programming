use rand::Rng; // Import the Rng trait for random number generation

fn generate_random_graph(num_nodes: usize, num_edges: usize) -> Vec<(usize, usize)> {
    // Defines a function named generate_random_graph.
    // It takes two parameters: num_nodes (number of nodes) and num_edges (number of edges).
    // It returns a vector of tuples, where each tuple represents an edge (node1, node2).

    let mut rng = rand::thread_rng(); // Creates a random number generator.
    // rand::thread_rng() gets a thread-local random number generator.
    // 'mut' makes the rng variable mutable, since we will use it to generate different random numbers.

    let mut edges = Vec::new(); // Creates an empty vector to store the edges.
    // Vec::new() initializes a new, empty vector.
    // 'mut' makes edges mutable, since we will add edges to it.

    for _ in 0..num_edges {
        // Starts a loop that iterates num_edges times.
        // '_' is used when we don't need the loop variable itself.

        let node1 = rng.gen_range(0..num_nodes); // Generates a random node index.
        // rng.gen_range(0..num_nodes) generates a random number within the specified range (0 to num_nodes - 1).

        let node2 = rng.gen_range(0..num_nodes); // Generates another random node index.
        // Generates another random node index, same as node1.

        if node1 != node2 { // Checks if the nodes are different to avoid self-loops.
            // A self-loop is an edge from a node to itself.

            edges.push((node1, node2)); // Adds the edge to the edges vector.
            // edges.push((node1, node2)) adds a tuple (node1, node2) to the end of the edges vector.
        }
    }

    edges // Returns the vector of edges.
}

fn main() {
    let num_nodes = 5; // Sets the number of nodes in the graph.
    let num_edges = 8; // Sets the number of edges in the graph.

    let graph = generate_random_graph(num_nodes, num_edges); // Generates the random graph.

    println!("Generated Random Graph:"); // Prints a message to the console.

    for (node1, node2) in graph {
        // Iterates through the edges in the graph vector.
        // Each edge is a tuple (node1, node2).

        println!("Edge: {} -> {}", node1, node2); // Prints each edge.
        // Prints the nodes that are connected by the edge.
    }
}
