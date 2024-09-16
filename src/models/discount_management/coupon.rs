// src/models/discount_management/coupon.rs

/// Representa um cupom de desconto.
#[derive(Debug, Clone)]
pub struct Coupon {
    pub code: String,
    pub discount_percentage: f64,
    pub expiration_date: Option<String>, // Formato: "YYYY-MM-DD"
}

impl Coupon {
    /// Cria um novo cupom de desconto.
    pub fn new(code: String, discount_percentage: f64, expiration_date: Option<String>) -> Self {
        Self {
            code,
            discount_percentage,
            expiration_date,
        }
    }

    /// Verifica se o cupom está expirado.
    pub fn is_expired(&self, current_date: &str) -> bool {
        if let Some(expiration) = &self.expiration_date {
            return current_date > expiration.as_str(); // Correção aqui
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coupon_creation() {
        let coupon = Coupon::new("SAVE20".to_string(), 20.0, Some("2023-12-31".to_string()));
        assert_eq!(coupon.code, "SAVE20");
        assert_eq!(coupon.discount_percentage, 20.0);
        assert_eq!(coupon.expiration_date.as_ref().unwrap(), "2023-12-31");
    }

    #[test]
    fn test_coupon_expiration() {
        let coupon = Coupon::new("SAVE20".to_string(), 20.0, Some("2023-12-31".to_string()));
        assert!(!coupon.is_expired("2023-12-30"));
        assert!(coupon.is_expired("2024-01-01"));
    }
}