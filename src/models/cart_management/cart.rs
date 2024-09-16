// src/models/cart_management/cart.rs

use std::collections::HashMap;
use crate::models::cart_management::cart_item::CartItem;

/// Representa o carrinho de compras.
#[derive(Debug, Default)]
pub struct Cart {
    items: HashMap<String, CartItem>,
}

impl Cart {
    /// Cria um novo carrinho vazio.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Adiciona um item ao carrinho.
    pub fn add_item(&mut self, product_id: String, quantity: u32, price: f64) {
        let item = self.items.entry(product_id.clone()).or_insert(CartItem::new(product_id, 0, price));
        item.quantity += quantity;
    }

    /// Remove um item do carrinho.
    pub fn remove_item(&mut self, product_id: &str) {
        self.items.remove(product_id);
    }

    /// Atualiza a quantidade de um item no carrinho.
    pub fn update_item_quantity(&mut self, product_id: &str, quantity: u32) {
        if let Some(item) = self.items.get_mut(product_id) {
            item.quantity = quantity;
        }
    }

    /// Calcula o total do carrinho.
    pub fn total(&self) -> f64 {
        self.items.values().map(|item| item.total()).sum()
    }

    /// Lista todos os itens no carrinho.
    pub fn list_items(&self) -> Vec&CartItem> {
        self.items.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_management() {
        let mut cart = Cart::new();
        cart.add_item("product_1".to_string(), 2, 10.0);
        cart.add_item("product_2".to_string(), 1, 20.0);
        
        assert_eq!(cart.total(), 40.0);
        
        cart.update_item_quantity("product_1", 3);
        assert_eq!(cart.total(), 50.0);
        
        cart.remove_item("product_2");
        assert_eq!(cart.total(), 30.0);
        
        let items = cart.list_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].product_id, "product_1");
    }
}