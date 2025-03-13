# Random Graph Generator in Rust

This project generates a random graph in Rust, represented as a list of edges.

## Description

The `generate_random_graph` function creates a graph with a specified number of nodes and edges. It uses the `rand` crate to generate random node indices for each edge, ensuring that no self-loops are created. The graph is represented as a vector of tuples, where each tuple `(node1, node2)` indicates an edge from `node1` to `node2`.

## Getting Started

### Prerequisites

* Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).

### Installation

1.  Clone the repository:

    ```bash
    git clone [repository_url]
    cd random-graph-generator
    ```

2.  Add the `rand` crate as a dependency. Open your `Cargo.toml` file and add the following line to the `[dependencies]` section:

    ```toml
    rand = "0.8"
    ```

3.  Build and run the project:

    ```bash
    cargo run
    ```

### Example Usage

The `main.rs` file contains an example of how to use the `generate_random_graph` function:

```rust
use rand::Rng;

fn generate_random_graph(num_nodes: usize, num_edges: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut edges = Vec::new();

    for _ in 0..num_edges {
        let node1 = rng.gen_range(0..num_nodes);
        let node2 = rng.gen_range(0..num_nodes);

        if node1 != node2 {
            edges.push((node1, node2));
        }
    }

    edges
}

fn main() {
    let num_nodes = 5;
    let num_edges = 8;

    let graph = generate_random_graph(num_nodes, num_edges);

    println!("Generated Random Graph:");

    for (node1, node2) in graph {
        println!("Edge: {} -> {}", node1, node2);
    }
}
