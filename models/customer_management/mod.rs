// src/models/customer_management/mod.rs

pub mod address;
pub mod customer;
pub mod review;
pub mod wishlist;

pub use address::Address;
pub use customer::Customer;
pub use review::Review;
pub use wishlist::Wishlist;