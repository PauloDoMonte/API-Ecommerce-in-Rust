// src/models/order_management/payment.rs

/// Representa um pagamento de um pedido.
#[derive(Debug, Clone)]
pub struct Payment {
    pub method: String, // Método de pagamento (ex: "cartão de crédito", "PayPal")
    pub amount: f64,    // Valor pago
}

impl Payment {
    /// Cria um novo pagamento.
    pub fn new(method: String, amount: f64) -> Self {
        Self { method, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_creation() {
        let payment = Payment::new("credit_card".to_string(), 100.0);
        assert_eq!(payment.method, "credit_card");
        assert_eq!(payment.amount, 100.0);
    }
}