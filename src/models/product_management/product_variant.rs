// src/models/product_management/product_variant.rs

/// Representa uma variante de um produto.
#[derive(Debug, Clone)]
pub struct ProductVariant {
    pub id: String,
    pub name: String, // Nome da variante (ex: "128GB", "Black")
    pub price: f64,   // Preço da variante
}

impl ProductVariant {
    /// Cria uma nova variante de produto.
    pub fn new(id: String, name: String, price: f64) -> Self {
        Self {
            id,
            name,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_variant_creation() {
        let variant = ProductVariant::new("variant_1".to_string(), "128GB".to_string(), 699.99);
        assert_eq!(variant.id, "variant_1");
        assert_eq!(variant.name, "128GB");
        assert_eq!(variant.price, 699.99);
    }
}