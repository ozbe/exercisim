pub mod graph_items;

use graph_items::edge::Edge;
use graph_items::node::Node;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub attrs: HashMap<String, String>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_attrs<I, K, V>(mut self, attrs: I) -> Graph
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: ToString,
        V: ToString,
    {
        self.attrs = attrs
            .into_iter()
            .map(|i| {
                let (k, v) = i.borrow();
                (k.to_string(), v.to_string())
            })
            .collect();
        self
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            attrs: HashMap::new(),
            nodes: vec![],
            edges: vec![],
        }
    }
}
