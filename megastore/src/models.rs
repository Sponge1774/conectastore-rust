//! Modelos de domínio: vértices (Produto, Cliente, Categoria) e tipos de aresta.

/// Identificador único de um vértice no grafo (independe do tipo de entidade).
pub type NodeId = u32;

/// Tipo de vértice representado no grafo.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Product,
    Client,
    Category,
}

/// Vértice genérico do grafo do ConectaStore.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub label: String,
}

/// Tipo de relação representada por uma aresta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeKind {
    /// Cliente -> Produto (compra realizada)
    Purchase,
    /// Cliente -> Produto (avaliação dada)
    Rating,
    /// Produto -> Produto (similaridade de conteúdo/categoria)
    Similarity,
    /// Produto -> Categoria (pertencimento)
    BelongsTo,
}

/// Aresta ponderada e dirigida do grafo.
#[derive(Debug, Clone)]
pub struct Edge {
    pub target: NodeId,
    pub kind: EdgeKind,
    /// Peso da relação (ex.: nota de avaliação normalizada, frequência de compra,
    /// grau de similaridade). Usado para ordenar recomendações.
    pub weight: f64,
}

/// Registro de produto usado no cadastro/consulta independente do grafo
/// (acesso O(1) por identificador via HashMap, conforme exigido no enunciado).
#[derive(Debug, Clone)]
pub struct Product {
    pub id: NodeId,
    pub name: String,
    pub category_id: NodeId,
    pub price: f64,
}
