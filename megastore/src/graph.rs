//! Grafo dirigido e ponderado, representado por lista de adjacência.
//!
//! Escolha de projeto: lista de adjacência em vez de matriz de adjacência.
//! Justificativa (detalhada no relatório teórico, seção 3.4): o catálogo da
//! MegaStore é esparso (cada cliente/produto se relaciona com uma fração
//! pequena do total de vértices), então a lista de adjacência custa
//! O(V + E) em espaço, contra O(V²) da matriz — decisivo para milhões de
//! produtos.

use crate::models::{Edge, EdgeKind, Node, NodeId, NodeKind};
use std::collections::HashMap;

pub struct Graph {
    /// Registro de vértices por id — acesso O(1).
    nodes: HashMap<NodeId, Node>,
    /// Lista de adjacência: cada vértice aponta para suas arestas de saída.
    adjacency: HashMap<NodeId, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Cria o grafo já com capacidade reservada para `n_nodes` vértices,
    /// evitando realocações e rehashing incrementais dos HashMaps durante
    /// a construção — mitigação discutida na Seção 6.3 do relatório para
    /// o crescimento superlinear observado no tempo de construção.
    pub fn with_capacity(n_nodes: usize) -> Self {
        Graph {
            nodes: HashMap::with_capacity(n_nodes),
            adjacency: HashMap::with_capacity(n_nodes),
        }
    }

    /// Insere um vértice. O(1) amortizado.
    pub fn add_node(&mut self, id: NodeId, kind: NodeKind, label: &str) {
        self.nodes.insert(
            id,
            Node {
                id,
                kind,
                label: label.to_string(),
            },
        );
        self.adjacency.entry(id).or_insert_with(Vec::new);
    }

    /// Insere uma aresta dirigida e ponderada de `from` para `to`. O(1) amortizado.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, kind: EdgeKind, weight: f64) {
        self.adjacency
            .entry(from)
            .or_insert_with(Vec::new)
            .push(Edge {
                target: to,
                kind,
                weight,
            });
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn neighbors(&self, id: NodeId) -> &[Edge] {
        self.adjacency
            .get(&id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(|v| v.len()).sum()
    }

    pub fn nodes_of_kind(&self, kind: &NodeKind) -> Vec<NodeId> {
        self.nodes
            .values()
            .filter(|n| &n.kind == kind)
            .map(|n| n.id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_and_edge() {
        let mut g = Graph::new();
        g.add_node(1, NodeKind::Client, "Cliente 1");
        g.add_node(2, NodeKind::Product, "Produto A");
        g.add_edge(1, 2, EdgeKind::Purchase, 1.0);

        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
        assert_eq!(g.neighbors(1).len(), 1);
        assert_eq!(g.neighbors(1)[0].target, 2);
    }

    #[test]
    fn neighbors_of_isolated_node_is_empty() {
        let mut g = Graph::new();
        g.add_node(1, NodeKind::Product, "Produto A");
        assert!(g.neighbors(1).is_empty());
    }

    #[test]
    fn neighbors_of_unknown_node_is_empty() {
        let g = Graph::new();
        assert!(g.neighbors(999).is_empty());
    }
}
