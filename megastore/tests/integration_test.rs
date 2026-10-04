//! Testes de integração: validam o fluxo completo — cadastro de produtos,
//! construção do grafo e geração de recomendações — como um usuário real
//! do sistema utilizaria.

use megastore::dataset::build_synthetic;
use megastore::graph::Graph;
use megastore::models::{EdgeKind, NodeKind, Product};
use megastore::recommend::recommend;
use megastore::store::Catalog;

#[test]
fn fluxo_completo_cadastro_grafo_recomendacao() {
    let mut graph = Graph::new();
    let mut catalog = Catalog::new();

    graph.add_node(1, NodeKind::Client, "Cliente 1");
    graph.add_node(100, NodeKind::Product, "Teclado Mecânico");
    graph.add_node(101, NodeKind::Product, "Mouse Gamer");

    catalog.register(Product {
        id: 100,
        name: "Teclado Mecânico".into(),
        category_id: 0,
        price: 350.0,
    });
    catalog.register(Product {
        id: 101,
        name: "Mouse Gamer".into(),
        category_id: 0,
        price: 180.0,
    });

    graph.add_edge(1, 100, EdgeKind::Purchase, 1.0);
    graph.add_edge(100, 101, EdgeKind::Similarity, 0.9);

    let recs = recommend(&graph, 1, 2, 5);
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].product_id, 101);
    assert!(catalog.get(101).is_some());
}

#[test]
fn dataset_sintetico_gera_volume_esperado_de_vertices() {
    let (graph, catalog) = build_synthetic(50, 200, 10, 4, 2, 7);

    // clientes + produtos + categorias
    assert_eq!(graph.node_count(), 50 + 200 + 10);
    assert_eq!(catalog.len(), 200);
    assert!(graph.edge_count() > 0);
}

#[test]
fn recomendacao_em_grafo_grande_nao_gera_duplicados() {
    let (graph, _catalog) = build_synthetic(200, 500, 20, 6, 3, 99);
    let recs = recommend(&graph, 10_000_000, 2, 20);

    let mut ids: Vec<_> = recs.iter().map(|r| r.product_id).collect();
    let before = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(before, ids.len());
}
