// src/models/shipping_management/shipping_method.rs

/// Representa um método de envio.
#[derive(Debug, Clone)]
pub struct ShippingMethod {
    pub id: String,
    pub name: String, // Nome do método de envio (ex: "Padrão", "Expresso")
    pub cost: f64,    // Custo do envio
    pub estimated_delivery_time: String, // Tempo estimado de entrega
}

impl ShippingMethod {
    /// Cria um novo método de envio.
    pub fn new(id: String, name: String, cost: f64, estimated_delivery_time: String) -> Self {
        Self {
            id,
            name,
            cost,
            estimated_delivery_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shipping_method_creation() {
        let shipping_method = ShippingMethod::new(
            "method_1".to_string(),
            "Express".to_string(),
            15.0,
            "1-2 dias".to_string(),
        );
        assert_eq!(shipping_method.id, "method_1");
        assert_eq!(shipping_method.name, "Express");
        assert_eq!(shipping_method.cost, 15.0);
        assert_eq!(shipping_method.estimated_delivery_time, "1-2 dias");
    }
}