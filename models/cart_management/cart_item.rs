Copy Code
// src/models/cart_management/cart_item.rs

/// Representa um item no carrinho de compras.
#[derive(Debug, Clone)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f64,
}

impl CartItem {
    /// Cria um novo item de carrinho.
    pub fn new(product_id: String, quantity: u32, price: f64) -> Self {
        Self {
            product_id,
            quantity,
            price,
        }
    }

    /// Calcula o total para este item.
    pub fn total(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_item() {
        let item = CartItem::new("product_1".to_string(), 2, 10.0);
        assert_eq!(item.total(), 20.0);
    }
}