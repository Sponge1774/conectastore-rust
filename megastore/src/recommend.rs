//! Algoritmo de recomendação por percurso em largura (BFS).
//!
//! Justificativa da escolha de BFS em vez de DFS (detalhada no relatório,
//! seção 4.1): recomendações devem priorizar produtos mais "próximos" do
//! cliente (menor número de saltos no grafo: compra direta > similaridade
//! de 2º grau > 3º grau). BFS explora por camadas de distância, o que
//! mapeia diretamente essa noção de relevância decrescente. DFS não dá
//! essa garantia de ordem por proximidade.

use crate::graph::Graph;
use crate::models::{EdgeKind, NodeId, NodeKind};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub product_id: NodeId,
    pub score: f64,
    pub distance: u32,
}

/// Gera recomendações de produtos a partir de um vértice de origem
/// (cliente ou produto), percorrendo o grafo em largura até `max_depth`
/// saltos e agregando peso das arestas percorridas.
///
/// Complexidade: O(V + E) no pior caso (cada vértice e aresta visitados
/// no máximo uma vez), pois `visited` garante que nenhum vértice é
/// reprocessado — este é também o mecanismo de prevenção de
/// recomendações duplicadas exigido no enunciado.
pub fn recommend(
    graph: &Graph,
    start: NodeId,
    max_depth: u32,
    limit: usize,
) -> Vec<Recommendation> {
    let owned = products_already_interacted(graph, start);

    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut queue: VecDeque<(NodeId, u32, f64)> = VecDeque::new();
    let mut recommended: HashSet<NodeId> = HashSet::new(); // prevenção de duplicados
    let mut results: Vec<Recommendation> = Vec::new();

    visited.insert(start);
    queue.push_back((start, 0, 1.0));

    while let Some((current, depth, acc_weight)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        for edge in graph.neighbors(current) {
            if visited.contains(&edge.target) {
                continue;
            }
            visited.insert(edge.target);

            let combined_weight = acc_weight * edge.weight;

            if let Some(node) = graph.node(edge.target) {
                if node.kind == NodeKind::Product
                    && edge.target != start
                    && !owned.contains(&edge.target)
                    && !recommended.contains(&edge.target)
                {
                    recommended.insert(edge.target);
                    results.push(Recommendation {
                        product_id: edge.target,
                        score: combined_weight,
                        distance: depth + 1,
                    });
                }
            }

            queue.push_back((edge.target, depth + 1, combined_weight));
        }
    }

    // Ordena por relevância: menor distância primeiro, depois maior score.
    results.sort_by(|a, b| {
        a.distance
            .cmp(&b.distance)
            .then(b.score.partial_cmp(&a.score).unwrap())
    });
    results.truncate(limit);
    results
}

/// Variante didática usando busca em profundidade (DFS), mantida para
/// comparação empírica de desempenho (seção 6 do relatório). Usa a mesma
/// estratégia de prevenção de duplicados via HashSet.
pub fn recommend_dfs(
    graph: &Graph,
    start: NodeId,
    max_depth: u32,
    limit: usize,
) -> Vec<Recommendation> {
    let owned = products_already_interacted(graph, start);

    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut recommended: HashSet<NodeId> = HashSet::new();
    let mut results: Vec<Recommendation> = Vec::new();

    visited.insert(start);
    dfs_visit(
        graph,
        start,
        0,
        max_depth,
        1.0,
        start,
        &owned,
        &mut visited,
        &mut recommended,
        &mut results,
    );

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(limit);
    results
}

fn dfs_visit(
    graph: &Graph,
    current: NodeId,
    depth: u32,
    max_depth: u32,
    acc_weight: f64,
    start: NodeId,
    owned: &HashSet<NodeId>,
    visited: &mut HashSet<NodeId>,
    recommended: &mut HashSet<NodeId>,
    results: &mut Vec<Recommendation>,
) {
    if depth >= max_depth {
        return;
    }
    for edge in graph.neighbors(current) {
        if visited.contains(&edge.target) {
            continue;
        }
        visited.insert(edge.target);
        let combined_weight = acc_weight * edge.weight;

        if let Some(node) = graph.node(edge.target) {
            if node.kind == NodeKind::Product
                && edge.target != start
                && !owned.contains(&edge.target)
                && !recommended.contains(&edge.target)
            {
                recommended.insert(edge.target);
                results.push(Recommendation {
                    product_id: edge.target,
                    score: combined_weight,
                    distance: depth + 1,
                });
            }
        }

        dfs_visit(
            graph,
            edge.target,
            depth + 1,
            max_depth,
            combined_weight,
            start,
            owned,
            visited,
            recommended,
            results,
        );
    }
}

/// Retorna o conjunto de produtos com os quais o vértice de origem já
/// interagiu diretamente (compra ou avaliação) — usado para não
/// recomendar de volta algo que o cliente já tem/avaliou.
fn products_already_interacted(graph: &Graph, start: NodeId) -> HashSet<NodeId> {
    graph
        .neighbors(start)
        .iter()
        .filter(|e| e.kind == EdgeKind::Purchase || e.kind == EdgeKind::Rating)
        .map(|e| e.target)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::models::{EdgeKind, NodeKind};

    fn sample_graph() -> Graph {
        let mut g = Graph::new();
        g.add_node(1, NodeKind::Client, "Cliente 1");
        g.add_node(10, NodeKind::Product, "Produto A");
        g.add_node(11, NodeKind::Product, "Produto B");
        g.add_node(12, NodeKind::Product, "Produto C");
        g.add_edge(1, 10, EdgeKind::Purchase, 1.0);
        g.add_edge(10, 11, EdgeKind::Similarity, 0.8);
        g.add_edge(10, 12, EdgeKind::Similarity, 0.5);
        g
    }

    #[test]
    fn recommends_products_reachable_from_client() {
        let g = sample_graph();
        let recs = recommend(&g, 1, 2, 10);
        let ids: Vec<NodeId> = recs.iter().map(|r| r.product_id).collect();
        assert!(ids.contains(&11));
        assert!(ids.contains(&12));
        assert!(!ids.contains(&10)); // produto já comprado não deve ser recomendado
    }

    #[test]
    fn respects_limit() {
        let g = sample_graph();
        let recs = recommend(&g, 1, 2, 1);
        assert_eq!(recs.len(), 1);
    }

    #[test]
    fn no_duplicate_recommendations() {
        let mut g = sample_graph();
        // Aresta redundante para o mesmo produto, simulando múltiplas relações.
        g.add_edge(1, 10, EdgeKind::Rating, 0.9);
        let recs = recommend(&g, 1, 3, 10);
        let mut ids: Vec<NodeId> = recs.iter().map(|r| r.product_id).collect();
        let before = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(before, ids.len());
    }

    #[test]
    fn bfs_and_dfs_reach_same_set_of_products() {
        let g = sample_graph();
        let mut bfs_ids: Vec<NodeId> = recommend(&g, 1, 2, 10)
            .iter()
            .map(|r| r.product_id)
            .collect();
        let mut dfs_ids: Vec<NodeId> = recommend_dfs(&g, 1, 2, 10)
            .iter()
            .map(|r| r.product_id)
            .collect();
        bfs_ids.sort();
        dfs_ids.sort();
        assert_eq!(bfs_ids, dfs_ids);
    }
}
