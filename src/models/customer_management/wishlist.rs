// src/models/customer_management/wishlist.rs

use std::collections::HashSet;

/// Representa uma lista de desejos de um cliente.
#[derive(Debug, Default)]
pub struct Wishlist {
    pub customer_id: String,
    pub items: HashSet<String>, // IDs dos produtos
}

impl Wishlist {
    /// Cria uma nova lista de desejos.
    pub fn new(customer_id: String) -> Self {
        Self {
            customer_id,
            items: HashSet::new(),
        }
    }

    /// Adiciona um item à lista de desejos.
    pub fn add_item(&mut self, product_id: String) {
        self.items.insert(product_id);
    }

    /// Remove um item da lista de desejos.
    pub fn remove_item(&mut self, product_id: &str) {
        self.items.remove(product_id);
    }

    /// Lista todos os itens na lista de desejos.
    pub fn list_items(&self) -> Vec<String> {
        self.items.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wishlist_management() {
        let mut wishlist = Wishlist::new("customer_1".to_string());
        wishlist.add_item("product_1".to_string());
        wishlist.add_item("product_2".to_string());

        assert_eq!(wishlist.items.len(), 2);
        
        wishlist.remove_item("product_1");
        assert_eq!(wishlist.items.len(), 1);
    }
}