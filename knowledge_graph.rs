use std::collections::HashMap;

// Define a struct for a node in the knowledge graph.
struct Node 
{
    id: String,
    properties: HashMap<String, String>,
}

// Define a struct for an edge in the knowledge graph.
struct Edge 
{
    source: String,
    target: String,
    relation: String,
}

// Define a struct for the knowledge graph itself.
struct KnowledgeGraph 
{
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
}

impl KnowledgeGraph 
{
    // Constructor for creating a new KnowledgeGraph.
    fn new() -> KnowledgeGraph 
    {
        KnowledgeGraph 
        {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    // Method to add a node to the graph.
    fn add_node(&mut self, node: Node) 
    {
        self.nodes.insert(node.id.clone(), node);
    }

    // Method to add an edge to the graph.
    fn add_edge(&mut self, edge: Edge) 
    {
        self.edges.push(edge);
    }

    // Method to retrieve a node by its ID.
    fn get_node(&self, node_id: &str) -> Option<&Node> 
    {
        self.nodes.get(node_id)
    }

    //Method to print the entire graph.
    fn print_graph(&self)
    {
        println!("Nodes:");
        for node in self.nodes.values()
        {
            println!("  ID: {}, Properties: {:?}", node.id, node.properties);
        }
        println!("Edges:");
        for edge in &self.edges{
            println!("  Source: {}, Target: {}, Relation: {}", edge.source, edge.target, edge.relation);
        }
    }
}

fn main() {
    let mut graph = KnowledgeGraph::new();

    // Create some nodes.
    let node1 = Node {
        id: "person1".to_string(),
        properties: HashMap::from([
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
        ]),
    };
    let node2 = Node {
        id: "city1".to_string(),
        properties: HashMap::from([("name".to_string(), "London".to_string())]),
    };
    let node3 = Node {
        id: "company1".to_string(),
        properties: HashMap::from([("name".to_string(), "Acme Corp".to_string())]),
    };

    // Add nodes to the graph.
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);

    // Create some edges.
    let edge1 = Edge {
        source: "person1".to_string(),
        target: "city1".to_string(),
        relation: "lives_in".to_string(),
    };

    let edge2 = Edge {
        source: "person1".to_string(),
        target: "company1".to_string(),
        relation: "works_at".to_string(),
    };

    // Add edges to the graph.
    graph.add_edge(edge1);
    graph.add_edge(edge2);

    // Retrieve a node.
    if let Some(node) = graph.get_node("person1") {
        println!("Retrieved node: ID: {}, Properties: {:?}", node.id, node.properties);
    }

    //print the entire graph.
    graph.print_graph();
}
