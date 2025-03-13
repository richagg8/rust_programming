# Random Graph Generator in Rust

This project generates a small, random undirected graph using Rust. It demonstrates basic graph generation and utilizes the `rand` crate for random number generation.

## Code Explanation

The Rust code consists of a function `generate_random_graph` that generates the graph and a `main` function that uses it to create and print a graph.

Here's a breakdown of the code:

* **`let mut edges = Vec::new();`**:
    * This line creates a mutable vector named `edges`. This vector will store the edges of the graph. Each edge is represented as a tuple of two `usize` values, where each `usize` represents a node index. `Vec::new()` initializes an empty vector.
* **`let mut rng = rand::thread_rng();`**:
    * This line creates a mutable random number generator named `rng`. `rand::thread_rng()` gets a thread-local random number generator, suitable for most cases. The `mut` keyword makes the random number generator mutable, as we need to generate new random numbers.
* **`while edges.len() < num_edges {`**:
    * This line starts a `while` loop that continues as long as the number of edges in the `edges` vector is less than the desired number of edges (`num_edges`).
* **`let node1 = rng.gen_range(0..num_nodes);` and `let node2 = rng.gen_range(0..num_nodes);`**:
    * These lines generate two random node indices. `rng.gen_range(0..num_nodes)` generates a random `usize` value in the range from 0 (inclusive) to `num_nodes` (exclusive). This ensures that the generated node indices are within the valid range for the graph.
* **`if node1 != node2 {`**:
    * This conditional statement checks if the two nodes are different. If they are the same, it would create a self-loop, which we are avoiding in this simple graph generation.
* **`if !edges.contains(&(node1, node2)) && !edges.contains(&(node2, node1)) {`**:
    * This conditional statement checks if the edge already exists in the `edges` vector. Because the graph is undirected, we check for both `(node1, node2)` and `(node2, node1)`. `edges.contains()` checks if a specific element exists in the vector. The `!` operator negates the result, so the condition is true if the edge does *not* exist.
* **`edges.push((node1, node2));`**:
    * If the edge is unique and the nodes are different, this line adds the edge to the `edges` vector. `edges.push()` appends the given element to the end of the vector.
* **`edges`**:
    * The function returns the vector containing the generated edges.
* **`let num_nodes = 5;` and `let num_edges = 7;`**:
    * These lines define the number of nodes and edges for the graph. You can change these values to generate graphs of different sizes.
* **`let graph = generate_random_graph(num_nodes, num_edges);`**:
    * This line calls the `generate_random_graph` function with the specified number of nodes and edges, and stores the resulting vector of edges in the `graph` variable.
* **`println!("Generated graph:");` and the `for` loop**:
    * These lines print the generated graph to the console. The `for` loop iterates through the `graph` vector, and for each edge, it prints the node indices separated by `" -- "`. This visualizes the connections in the graph.

## How to Run

1.  **Install Rust:**
    * If you don't have Rust installed, you can install it from [rustup.rs](https://rustup.rs).
2.  **Create a New Rust Project:**
    * Open a terminal and run `cargo new random_graph`.
    * Navigate to the project directory: `cd random_graph`.
3.  **Replace `src/main.rs` Contents:**
    * Replace the contents of `src/main.rs` with the Rust code provided.
4.  **Add `rand` Dependency:**
    * Open `Cargo.toml` and add the following line to the `[dependencies]` section:

    ```toml
    [dependencies]
    rand = "0.8"
    ```

5.  **Run the Code:**
    * In the terminal, run `cargo run`.
    * This will compile and run the Rust code, and you will see the generated random graph printed to the console.



