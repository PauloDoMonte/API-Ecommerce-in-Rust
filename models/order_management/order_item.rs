// src/models/order_management/order_item.rs

/// Representa um item em um pedido.
#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: u32,
    pub price: f64,
}

impl OrderItem {
    /// Cria um novo item de pedido.
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
    fn test_order_item_creation() {
        let item = OrderItem::new("product_1".to_string(), 2, 10.0);
        assert_eq!(item.total(), 20.0);
    }
}