//! Geração de dados sintéticos para simular catálogos da MegaStore em
//! diferentes volumes, usada na seção 6 (Desempenho e Escalabilidade) do
//! relatório. Não depende de nenhuma fonte externa de aleatoriedade
//! criptográfica — usa um gerador congruencial simples (determinístico,
//! reprodutível entre execuções).

use crate::graph::Graph;
use crate::models::{EdgeKind, NodeKind};
use crate::store::Catalog;
use crate::models::Product;

struct SimpleRng(u64);

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng(seed)
    }
    fn next(&mut self) -> u64 {
        // Gerador congruencial linear (parâmetros de Numerical Recipes).
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }
    fn range(&mut self, bound: u32) -> u32 {
        (self.next() % bound as u64) as u32
    }
}

/// Constrói um grafo + catálogo sintéticos com `n_clients` clientes,
/// `n_products` produtos e `n_categories` categorias, com `edges_per_client`
/// compras/avaliações aleatórias por cliente e `similarity_per_product`
/// arestas de similaridade por produto.
pub fn build_synthetic(
    n_clients: u32,
    n_products: u32,
    n_categories: u32,
    edges_per_client: u32,
    similarity_per_product: u32,
    seed: u64,
) -> (Graph, Catalog) {
    let mut rng = SimpleRng::new(seed);
    let total_nodes = (n_clients + n_products + n_categories) as usize;
    let mut graph = Graph::with_capacity(total_nodes);
    let mut catalog = Catalog::with_capacity(n_products as usize);

    // ids: categorias [0, n_categories) | produtos [1000, 1000+n_products)
    // | clientes [10_000_000, 10_000_000+n_clients)
    const PRODUCT_OFFSET: u32 = 1_000;
    const CLIENT_OFFSET: u32 = 10_000_000;

    for c in 0..n_categories {
        graph.add_node(c, NodeKind::Category, &format!("Categoria {c}"));
    }

    for p in 0..n_products {
        let id = PRODUCT_OFFSET + p;
        let category_id = rng.range(n_categories.max(1));
        graph.add_node(id, NodeKind::Product, &format!("Produto {p}"));
        graph.add_edge(id, category_id, EdgeKind::BelongsTo, 1.0);
        catalog.register(Product {
            id,
            name: format!("Produto {p}"),
            category_id,
            price: 10.0 + (rng.range(1000) as f64) / 10.0,
        });
    }

    // Arestas de similaridade produto-produto.
    for p in 0..n_products {
        let id = PRODUCT_OFFSET + p;
        for _ in 0..similarity_per_product {
            let other = PRODUCT_OFFSET + rng.range(n_products.max(1));
            if other != id {
                let weight = 0.3 + (rng.range(70) as f64) / 100.0;
                graph.add_edge(id, other, EdgeKind::Similarity, weight);
            }
        }
    }

    for c in 0..n_clients {
        let id = CLIENT_OFFSET + c;
        graph.add_node(id, NodeKind::Client, &format!("Cliente {c}"));
        for _ in 0..edges_per_client {
            let product = PRODUCT_OFFSET + rng.range(n_products.max(1));
            let weight = 0.5 + (rng.range(50) as f64) / 100.0;
            graph.add_edge(id, product, EdgeKind::Purchase, weight);
        }
    }

    (graph, catalog)
}
