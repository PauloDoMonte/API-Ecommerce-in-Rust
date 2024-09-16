// src/models/customer_management/review.rs

/// Representa uma avaliação de um produto por um cliente.
#[derive(Debug, Clone)]
pub struct Review {
    pub product_id: String,
    pub customer_id: String,
    pub rating: u8, // De 1 a 5
    pub comment: String,
}

impl Review {
    /// Cria uma nova avaliação.
    pub fn new(product_id: String, customer_id: String, rating: u8, comment: String) -> Self {
        Self {
            product_id,
            customer_id,
            rating,
            comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_creation() {
        let review = Review::new("product_1".to_string(), "customer_1".to_string(), 5, "Excellent product!".to_string());
        assert_eq!(review.rating, 5);
        assert_eq!(review.comment, "Excellent product!");
    }
}