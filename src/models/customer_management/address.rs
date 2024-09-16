// src/models/customer_management/address.rs

/// Representa um endereço de um cliente.
#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

impl Address {
    /// Cria um novo endereço.
    pub fn new(street: String, city: String, state: String, postal_code: String, country: String) -> Self {
        Self {
            street,
            city,
            state,
            postal_code,
            country,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let address = Address::new(
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "CA".to_string(),
            "12345".to_string(),
            "USA".to_string(),
        );
        assert_eq!(address.street, "123 Main St");
        assert_eq!(address.city, "Anytown");
        assert_eq!(address.state, "CA");
        assert_eq!(address.postal_code, "12345");
        assert_eq!(address.country, "USA");
    }
}