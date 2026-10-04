//! Cadastro e consulta de produtos. Mantém um HashMap independente do
//! grafo para acesso O(1) por identificador — o grafo responde "quem se
//! relaciona com quem", o catálogo responde "quais são os dados deste
//! produto", conforme separação de responsabilidades descrita na seção
//! 3.5 do relatório.

use crate::models::{NodeId, Product};
use std::collections::HashMap;

pub struct Catalog {
    products: HashMap<NodeId, Product>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            products: HashMap::new(),
        }
    }

    /// Cria o catálogo já com capacidade reservada para `n_products`,
    /// pelo mesmo motivo de `Graph::with_capacity`.
    pub fn with_capacity(n_products: usize) -> Self {
        Catalog {
            products: HashMap::with_capacity(n_products),
        }
    }

    /// Cadastra um produto. O(1) amortizado.
    pub fn register(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    /// Consulta um produto pelo identificador. O(1) amortizado.
    pub fn get(&self, id: NodeId) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn len(&self) -> usize {
        self.products.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_query_product() {
        let mut catalog = Catalog::new();
        catalog.register(Product {
            id: 1,
            name: "Notebook".to_string(),
            category_id: 100,
            price: 3500.0,
        });

        let found = catalog.get(1).expect("produto deveria existir");
        assert_eq!(found.name, "Notebook");
    }

    #[test]
    fn query_unknown_product_returns_none() {
        let catalog = Catalog::new();
        assert!(catalog.get(999).is_none());
    }
}
