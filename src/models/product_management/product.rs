// src/models/product_management/product.rs

use crate::models::product_management::category::Category;
use crate::models::product_management::product_variant::ProductVariant;

/// Representa um produto.
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Category,
    pub variants: Vec<ProductVariant>,
}

impl Product {
    /// Cria um novo produto.
    pub fn new(id: String, name: String, description: Option<String>, category: Category) -> Self {
        Self {
            id,
            name,
            description,
            category,
            variants: Vec::new(),
        }
    }

    /// Adiciona uma variante ao produto.
    pub fn add_variant(&mut self, variant: ProductVariant) {
        self.variants.push(variant);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::product_management::category::Category;
    use crate::models::product_management::product_variant::ProductVariant;

    #[test]
    fn test_product_creation() {
        let category = Category::new("cat_1".to_string(), "Electronics".to_string(), None);
        let mut product = Product::new("prod_1".to_string(), "Smartphone".to_string(), Some("Latest model".to_string()), category);
        
        assert_eq!(product.id, "prod_1");
        assert_eq!(product.name, "Smartphone");
        assert!(product.variants.is_empty());

        let variant = ProductVariant::new("variant_1".to_string(), "128GB".to_string(), 699.99);
        product.add_variant(variant);
        assert_eq!(product.variants.len(), 1);
    }
}