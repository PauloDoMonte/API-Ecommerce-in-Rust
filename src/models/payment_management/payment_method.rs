// src/models/payment_management/payment_method.rs

/// Representa um método de pagamento.
#[derive(Debug, Clone)]
pub struct PaymentMethod {
    pub method_type: String, // Ex: "Cartão de Crédito", "PayPal", "Boleto"
    pub details: String,      // Detalhes adicionais, como número do cartão ou e-mail do PayPal
}

impl PaymentMethod {
    /// Cria um novo método de pagamento.
    pub fn new(method_type: String, details: String) -> Self {
        Self {
            method_type,
            details,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_method_creation() {
        let payment_method = PaymentMethod::new("Credit Card".to_string(), "**** **** **** 1234".to_string());
        assert_eq!(payment_method.method_type, "Credit Card");
        assert_eq!(payment_method.details, "**** **** **** 1234");
    }
}