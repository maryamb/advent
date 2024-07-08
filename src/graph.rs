use std::collections::HashSet;

mod graph {

pub trait Node: PartialOrd + Clone + Eq + std::hash::Hash {}

pub struct Graph<N: Node, E> {
    nodes: Vec<N>,
    edges: Vec<(N, N, E)>, // (source, target, edge data)
}

impl<N: Node, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, source: N, target: N, edge: E) {
        self.edges.push((source, target, edge));
    }

    // DFS implementation
    pub fn dfs(&self, start: &N) -> Vec<N> {
        let mut visited = HashSet::new();
        let mut stack = vec![start.clone()];
        let mut result = Vec::new();

        while let Some(node) = stack.pop() {
            if visited.insert(node.clone()) {
                result.push(node.clone());
                for &(source, target, _) in &self.edges {
                    if source == node && !visited.contains(&target) {
                        stack.push(target.clone());
                    }
                }
            }
        }

        result
    }
}
}