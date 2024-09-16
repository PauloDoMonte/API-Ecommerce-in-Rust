// src/models/order_management/shipping.rs

/// Representa o envio de um pedido.
#[derive(Debug, Clone)]
pub struct Shipping {
    pub method: String, // Método de envio (ex: "Padrão", "Expresso")
    pub cost: f64,      // Custo do envio
}

impl Shipping {
    /// Cria um novo envio.
    pub fn new(method: String, cost: f64) -> Self {
        Self { method, cost }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shipping_creation() {
        let shipping = Shipping::new("Express".to_string(), 5.0);
        assert_eq!(shipping.method, "Express");
        assert_eq!(shipping.cost, 5.0);
    }
}