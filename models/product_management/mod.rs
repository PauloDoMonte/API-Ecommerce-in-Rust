// src/models/product_management/mod.rs

pub mod category;
pub mod inventory;
pub mod product;
pub mod product_variant;

pub use category::Category;
pub use inventory::Inventory;
pub use product::Product;
pub use product_variant::ProductVariant;