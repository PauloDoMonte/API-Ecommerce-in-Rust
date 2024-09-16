use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

impl Product {
    pub fn new(id: i32, name: String, description: String, price: f64, stock: i32) -> Self {
        Product {
            id,
            name,
            description,
            price,
            stock,
        }
    }

    pub fn update_stock(&mut self, new_stock: i32) {
        self.stock = new_stock;
    }

    pub fn apply_discount(&mut self, percentage: f64) {
        self.price = self.price - (self.price * percentage / 100.0);
    }
}
