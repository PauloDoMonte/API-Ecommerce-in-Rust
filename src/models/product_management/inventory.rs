// src/models/product_management/inventory.rs

use std::collections::HashMap;

/// Representa o inventário de produtos.
#[derive(Debug, Default)]
pub struct Inventory {
    pub stock: HashMap<String, u32>, // ID do produto e quantidade em estoque
}

impl Inventory {
    /// Cria um novo inventário vazio.
    pub fn new() -> Self {
        Self {
            stock: HashMap::new(),
        }
    }

    /// Adiciona um produto ao inventário.
    pub fn add_product(&mut self, product_id: String, quantity: u32) {
        let entry = self.stock.entry(product_id).or_insert(0);
        *entry += quantity;
    }

    /// Remove um produto do inventário.
    pub fn remove_product(&mut self, product_id: &str, quantity: u32) -> Result<(), String> {
        if let Some(stock) = self.stock.get_mut(product_id) {
            if *stock >= quantity {
                *stock -= quantity;
                Ok(())
            } else {
                Err("Quantidade insuficiente em estoque".to_string())
            }
        } else {
            Err("Produto não encontrado no inventário".to_string())
        }
    }

    /// Obtém a quantidade em estoque de um produto.
    pub fn get_stock(&self, product_id: &str) -> Option<u32> {
        self.stock.get(product_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_management() {
        let mut inventory = Inventory::new();
        inventory.add_product("product_1".to_string(), 10);
        assert_eq!(inventory.get_stock("product_1"), Some(10));

        inventory.remove_product("product_1", 5).unwrap();
        assert_eq!(inventory.get_stock("product_1"), Some(5));

        let result = inventory.remove_product("product_1", 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Quantidade insuficiente em estoque");
    }
}