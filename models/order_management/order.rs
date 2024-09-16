// src/models/order_management/order.rs

use crate::models::order_management::order_item::OrderItem;
use crate::models::order_management::payment::Payment;
use crate::models::order_management::shipping::Shipping;
use std::collections::HashMap;

/// Representa um pedido.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub items: Vec<OrderItem>,
    pub payment: Option<Payment>,
    pub shipping: Option<Shipping>,
}

impl Order {
    /// Cria um novo pedido.
    pub fn new(id: String) -> Self {
        Self {
            id,
            items: Vec::new(),
            payment: None,
            shipping: None,
        }
    }

    /// Adiciona um item ao pedido.
    pub fn add_item(&mut self, item: OrderItem) {
        self.items.push(item);
    }

    /// Calcula o total do pedido.
    pub fn total(&self) -> f64 {
        self.items.iter().map(|item| item.total()).sum()
    }

    /// Define o pagamento do pedido.
    pub fn set_payment(&mut self, payment: Payment) {
        self.payment = Some(payment);
    }

    /// Define o envio do pedido.
    pub fn set_shipping(&mut self, shipping: Shipping) {
        self.shipping = Some(shipping);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::order_management::payment::Payment;
    use crate::models::order_management::shipping::Shipping;

    #[test]
    fn test_order_management() {
        let mut order = Order::new("order_1".to_string());
        order.add_item(OrderItem::new("product_1".to_string(), 2, 10.0));
        order.add_item(OrderItem::new("product_2".to_string(), 1, 20.0));

        assert_eq!(order.total(), 40.0);

        order.set_payment(Payment::new("credit_card".to_string(), 40.0));
        order.set_shipping(Shipping::new("Express".to_string(), 5.0));

        assert!(order.payment.is_some());
        assert!(order.shipping.is_some());
    }
}