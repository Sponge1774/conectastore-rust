# ConectaStore (Rust)

Sistema de recomendação de produtos baseado em grafos, em Rust — grafo em
lista de adjacência, BFS/DFS e HashMap/HashSet. Projeto da disciplina
**Data Structure Strategy and Implementation** (UniFECAF).

> Repositório: `conectastore-rust` · Crate: `megastore` (nome interno do
> pacote em `Cargo.toml` — não precisa bater com o nome do repositório).

---

## Objetivo e funcionamento
Sistema de recomendação para a MegaStore que modela produtos, clientes e
categorias como vértices de um grafo dirigido e ponderado, e compras,
avaliações, similaridades e pertencimento a categoria como arestas. As
recomendações são geradas por busca em largura (BFS) a partir de um
cliente ou produto, priorizando itens mais próximos no grafo e evitando
duplicatas.

## Tecnologias e estruturas utilizadas
- **Linguagem**: Rust (edition 2021)
- **Grafo**: lista de adjacência (`HashMap<NodeId, Vec<Edge>>`), pré-alocada
  via `with_capacity` para reduzir custo de rehashing na construção
- **Matriz de adjacência**: implementada em `src/matrix.rs` apenas para
  comparação empírica de espaço com a lista (não é usada em produção —
  ver discussão no relatório, Seção 3.4)
- **Catálogo de produtos**: `HashMap<NodeId, Product>` para acesso O(1)
- **Recomendação**: BFS com `VecDeque` (fila) e `HashSet` (visitados e
  prevenção de duplicados); variante DFS incluída para comparação
- **Geração de dados sintéticos**: gerador congruencial linear próprio,
  sem dependências externas

## Instruções para compilação e execução
```bash
cargo build --release
./target/release/megastore
```
Alternativamente, o script `rodar_testes.sh` (na raiz do projeto) executa
compilação, testes e a demonstração/benchmarks em sequência, com pausas
para captura de prints:
```bash
./rodar_testes.sh
```

## Instruções para execução dos testes
```bash
cargo test --release
```
14 testes (11 unitários dos módulos `graph`, `recommend`, `store` e
`matrix`, mais 3 de integração), todos cobrindo cadastro, construção do
grafo, geração de recomendação e prevenção de duplicados.

## Exemplos de uso
A execução de `cargo run --release` roda automaticamente:
1. Uma demonstração pequena com 4 produtos e 1 cliente, mostrando cadastro,
   consulta direta e recomendação.
2. Um benchmark de desempenho com cinco volumes de dados (500 a 150 mil
   clientes), já com as estruturas pré-alocadas via `with_capacity`.
3. Uma comparação empírica de espaço entre lista e matriz de adjacência
   em pequena escala (até 3 mil vértices), demonstrando o crescimento
   quadrático da matriz.

## Arquitetura da solução
```
megastore/
├── src/
│   ├── main.rs        (demonstração + benchmarks)
│   ├── lib.rs          (exposição dos módulos para testes)
│   ├── models.rs       (Node, Edge, Product)
│   ├── graph.rs        (grafo em lista de adjacência, pré-alocável)
│   ├── matrix.rs        (matriz de adjacência — só para comparação)
│   ├── recommend.rs    (algoritmos BFS e DFS de recomendação)
│   ├── store.rs        (catálogo de produtos, pré-alocável)
│   └── dataset.rs       (gerador de dados sintéticos)
├── tests/
│   └── integration_test.rs
├── Cargo.toml
├── rodar_testes.sh    (script de testes/prints para o relatório)
└── README.md
```

## Resultados dos testes de desempenho
| Clientes | Produtos | Arestas    | Construção (ms) | Recomendação BFS (µs) |
|---------:|---------:|-----------:|-----------------:|------------------------:|
| 500      | 2.500    | 12.499     | 1,73             | 8,57                    |
| 2.000    | 10.000   | 49.999     | 6,52             | 12,21                   |
| 10.000   | 50.000   | 249.998    | 39,02            | 10,88                   |
| 50.000   | 250.000  | 1.249.999  | 207,35           | 11,35                   |
| 150.000  | 750.000  | 3.749.997  | 869,35           | 10,41                   |

## Comparação lista × matriz de adjacência (espaço ocupado)
| Vértices | Arestas | V²        | Lista (bytes) | Matriz (bytes) | Razão   |
|---------:|--------:|----------:|---------------:|----------------:|--------:|
| 110      | 316     | 12.100    | 8.464           | 96.800          | 11,4×   |
| 510      | 1.596   | 260.100   | 42.384          | 2.080.800       | 49,1×   |
| 1.510    | 4.796   | 2.280.100 | 127.184         | 18.240.800      | 143,4×  |
| 3.010    | 9.597   | 9.060.100 | 254.408         | 72.480.800      | 284,9×  |

Máquina de referência: notebook do autor (Samsung Expert, 2018, Intel Core
i7-8550U 1,80 GHz, 11 GiB de RAM, SSD de 428 GB, Pop!_OS), build
`--release`, otimização O3, execução via `rodar_testes.sh`. Discussão
completa no relatório técnico, Seções 3.4, 5.3 e 6.

## Relatório técnico
O relatório completo (fundamentação teórica, modelagem, análise de
complexidade e discussão de desempenho) está no PDF entregue junto com
este repositório — não incluído aqui por ser a Parte Teórica de uma
entrega acadêmica separada.

## Link do vídeo pitch
A preencher após a gravação (até 4 minutos), publicado no YouTube como
não listado ou em plataforma equivalente.
