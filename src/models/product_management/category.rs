// src/models/product_management/category.rs

/// Representa uma categoria de produtos.
#[derive(Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    /// Cria uma nova categoria.
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_creation() {
        let category = Category::new("cat_1".to_string(), "Electronics".to_string(), Some("Devices and gadgets".to_string()));
        assert_eq!(category.id, "cat_1");
        assert_eq!(category.name, "Electronics");
        assert_eq!(category.description.as_ref().unwrap(), "Devices and gadgets");
    }
}