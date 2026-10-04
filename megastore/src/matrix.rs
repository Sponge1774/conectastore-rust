//! Matriz de adjacência — implementada exclusivamente para permitir a
//! comparação empírica com a lista de adjacência (Seções 3.4 e 5.3 do
//! relatório). NÃO é a estrutura usada em produção pelo ConectaStore:
//! seu custo de espaço O(V²) a torna inviável para o volume real de
//! produtos da MegaStore, como os próprios números desta comparação
//! demonstram.

use crate::models::NodeId;

pub struct AdjacencyMatrix {
    /// Mapeia NodeId externo -> índice de linha/coluna na matriz.
    index_of: std::collections::HashMap<NodeId, usize>,
    /// Matriz V×V "achatada" em um único vetor (linha-major), guardando
    /// o peso da aresta (0.0 = ausência de aresta).
    weights: Vec<f64>,
    size: usize,
}

impl AdjacencyMatrix {
    /// Constrói a matriz para exatamente `node_ids` vértices. O vetor de
    /// pesos ocupa size*size posições de f64 (8 bytes cada),
    /// independentemente de quantas arestas realmente existirem.
    pub fn new(node_ids: &[NodeId]) -> Self {
        let size = node_ids.len();
        let mut index_of = std::collections::HashMap::with_capacity(size);
        for (i, &id) in node_ids.iter().enumerate() {
            index_of.insert(id, i);
        }
        AdjacencyMatrix {
            index_of,
            weights: vec![0.0; size * size],
            size,
        }
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: f64) {
        if let (Some(&i), Some(&j)) = (self.index_of.get(&from), self.index_of.get(&to)) {
            self.weights[i * self.size + j] = weight;
        }
    }

    pub fn weight(&self, from: NodeId, to: NodeId) -> f64 {
        match (self.index_of.get(&from), self.index_of.get(&to)) {
            (Some(&i), Some(&j)) => self.weights[i * self.size + j],
            _ => 0.0,
        }
    }

    /// Bytes efetivamente ocupados pelo vetor de pesos (sem contar o
    /// índice auxiliar) — usado para a comparação de espaço na Seção 5.3.
    pub fn bytes_used(&self) -> usize {
        self.weights.len() * std::mem::size_of::<f64>()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_reads_edge_weight() {
        let ids = [1, 2, 3];
        let mut m = AdjacencyMatrix::new(&ids);
        m.add_edge(1, 2, 0.75);
        assert_eq!(m.weight(1, 2), 0.75);
        assert_eq!(m.weight(2, 1), 0.0); // dirigido: sentido contrário não tem peso
    }

    #[test]
    fn space_grows_quadratically_with_vertex_count() {
        let ids_small: Vec<NodeId> = (0..100).collect();
        let ids_large: Vec<NodeId> = (0..1000).collect();
        let m_small = AdjacencyMatrix::new(&ids_small);
        let m_large = AdjacencyMatrix::new(&ids_large);
        // 10x mais vértices -> ~100x mais bytes (V²), não 10x (V).
        assert_eq!(m_large.bytes_used(), m_small.bytes_used() * 100);
    }
}
