// src/models/discount_management/discount.rs

/// Representa um desconto aplicado a um produto ou carrinho.
#[derive(Debug, Clone)]
pub struct Discount {
    pub discount_type: DiscountType,
    pub amount: f64, // Valor do desconto
}

#[derive(Debug, Clone)]
pub enum DiscountType {
    Percentage, // Desconto percentual
    Fixed,      // Desconto fixo
}

impl Discount {
    /// Cria um novo desconto.
    pub fn new(discount_type: DiscountType, amount: f64) -> Self {
        Self {
            discount_type,
            amount,
        }
    }

    /// Aplica o desconto a um valor total.
    pub fn apply(&self, total: f64) -> f64 {
        match self.discount_type {
            DiscountType::Percentage => total - (total * (self.amount / 100.0)),
            DiscountType::Fixed => (total - self.amount).max(0.0), // Não permitir valores negativos
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount_application() {
        let percentage_discount = Discount::new(DiscountType::Percentage, 10.0);
        assert_eq!(percentage_discount.apply(100.0), 90.0);

        let fixed_discount = Discount::new(DiscountType::Fixed, 15.0);
        assert_eq!(fixed_discount.apply(100.0), 85.0);
        assert_eq!(fixed_discount.apply(10.0), 0.0); // Não permitir valores negativos
    }
}