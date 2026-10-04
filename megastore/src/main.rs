use megastore::dataset;
use megastore::graph;
use megastore::matrix;
use megastore::models;
use megastore::recommend;
use megastore::store;
use megastore::models::NodeKind;
use std::time::Instant;

fn main() {
    println!("=== ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos ===\n");

    demo_pequena();
    println!();
    benchmark_volumes();
    println!();
    compare_matrix_vs_list();
}

/// Demonstração funcional com um grafo pequeno e legível, mostrando
/// cadastro, consulta e recomendação — usada nas capturas de tela do
/// relatório (Apêndice B).
fn demo_pequena() {
    let (mut graph, mut catalog) = (graph::Graph::new(), store::Catalog::new());

    graph.add_node(0, NodeKind::Category, "Eletrônicos");
    graph.add_node(1, NodeKind::Category, "Livros");

    graph.add_node(1000, NodeKind::Product, "Fone Bluetooth");
    graph.add_node(1001, NodeKind::Product, "Carregador USB-C");
    graph.add_node(1002, NodeKind::Product, "Livro de Rust");
    graph.add_node(1003, NodeKind::Product, "Mousepad");

    catalog.register(models::Product {
        id: 1000,
        name: "Fone Bluetooth".into(),
        category_id: 0,
        price: 199.90,
    });
    catalog.register(models::Product {
        id: 1001,
        name: "Carregador USB-C".into(),
        category_id: 0,
        price: 49.90,
    });
    catalog.register(models::Product {
        id: 1002,
        name: "Livro de Rust".into(),
        category_id: 1,
        price: 89.90,
    });
    catalog.register(models::Product {
        id: 1003,
        name: "Mousepad".into(),
        category_id: 0,
        price: 29.90,
    });

    graph.add_edge(1000, 0, models::EdgeKind::BelongsTo, 1.0);
    graph.add_edge(1001, 0, models::EdgeKind::BelongsTo, 1.0);
    graph.add_edge(1002, 1, models::EdgeKind::BelongsTo, 1.0);
    graph.add_edge(1003, 0, models::EdgeKind::BelongsTo, 1.0);

    graph.add_edge(1000, 1001, models::EdgeKind::Similarity, 0.8);
    graph.add_edge(1000, 1003, models::EdgeKind::Similarity, 0.6);
    graph.add_edge(1001, 1003, models::EdgeKind::Similarity, 0.5);

    graph.add_node(50, NodeKind::Client, "Cliente Eduardo");
    graph.add_edge(50, 1000, models::EdgeKind::Purchase, 1.0);
    graph.add_edge(50, 1002, models::EdgeKind::Rating, 0.9);

    println!("Catálogo: {} produtos cadastrados.", catalog.len());
    println!(
        "Consulta direta (id 1002): {:?}\n",
        catalog.get(1002).map(|p| &p.name)
    );

    let recs = recommend::recommend(&graph, 50, 2, 5);
    println!("Recomendações (BFS) para o Cliente Eduardo:");
    for r in &recs {
        let nome = catalog
            .get(r.product_id)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        println!(
            "  - {} (score={:.2}, distância={})",
            nome, r.score, r.distance
        );
    }
}

/// Mede o tempo de construção do grafo (já com HashMap pré-alocado via
/// with_capacity) e de geração de recomendações para diferentes volumes
/// de dados — dados usados na seção 6 do relatório (Desempenho e
/// Escalabilidade). Mais pontos de volume que a versão anterior, para
/// sustentar melhor a curva de crescimento.
fn benchmark_volumes() {
    let volumes = [
        (500u32, 2_500u32),
        (2_000, 10_000),
        (10_000, 50_000),
        (50_000, 250_000),
        (150_000, 750_000),
    ];

    println!("=== Benchmark: tempo de construção (com pré-alocação) e recomendação por volume ===");
    println!(
        "{:>12} | {:>12} | {:>10} | {:>14} | {:>16}",
        "clientes", "produtos", "arestas", "constr.(ms)", "recom. BFS(µs)"
    );

    for (n_clients, n_products) in volumes {
        let start_build = Instant::now();
        let (graph, _catalog) =
            dataset::build_synthetic(n_clients, n_products, 50, 5, 3, 42);
        let build_time = start_build.elapsed();

        let sample_client = 10_000_000; // primeiro cliente gerado
        let start_rec = Instant::now();
        let _recs = recommend::recommend(&graph, sample_client, 2, 10);
        let rec_time = start_rec.elapsed();

        println!(
            "{:>12} | {:>12} | {:>10} | {:>14.2} | {:>16.2}",
            n_clients,
            n_products,
            graph.edge_count(),
            build_time.as_secs_f64() * 1000.0,
            rec_time.as_secs_f64() * 1_000_000.0
        );
    }
}

/// Compara empiricamente lista de adjacência × matriz de adjacência em
/// escala pequena (a matriz não é viável em escala real — ver Seção 3.4).
/// Mede o espaço realmente ocupado por cada representação para o mesmo
/// conjunto de vértices e arestas.
fn compare_matrix_vs_list() {
    let volumes = [(20u32, 80u32), (100, 400), (300, 1_200), (600, 2_400)];

    println!("=== Comparação lista × matriz de adjacência (espaço ocupado) ===");
    println!(
        "{:>10} | {:>10} | {:>10} | {:>16} | {:>16} | {:>10}",
        "vértices", "arestas", "V²", "lista (bytes)", "matriz (bytes)", "razão"
    );

    for (n_clients, n_products) in volumes {
        let (graph, _catalog) = dataset::build_synthetic(n_clients, n_products, 10, 4, 2, 7);

        let node_ids: Vec<models::NodeId> = (0..10)
            .chain(1_000..1_000 + n_products)
            .chain(10_000_000..10_000_000 + n_clients)
            .collect();
        let mut mat = matrix::AdjacencyMatrix::new(&node_ids);
        for &id in &node_ids {
            for edge in graph.neighbors(id) {
                mat.add_edge(id, edge.target, edge.weight);
            }
        }

        let v = graph.node_count();
        let e = graph.edge_count();
        // Estimativa conservadora do custo da lista: um ponteiro/offset por
        // vértice (8 bytes) + estrutura Edge por aresta (target u32 + kind
        // enum + weight f64 ≈ 24 bytes com alinhamento).
        let list_bytes = v * 8 + e * 24;
        let matrix_bytes = mat.bytes_used();

        println!(
            "{:>10} | {:>10} | {:>10} | {:>16} | {:>16} | {:>9.1}x",
            v,
            e,
            v * v,
            list_bytes,
            matrix_bytes,
            matrix_bytes as f64 / list_bytes as f64
        );
    }
}
