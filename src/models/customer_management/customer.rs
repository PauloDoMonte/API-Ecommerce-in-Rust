// src/models/customer_management/customer.rs

use rocket::serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;

use crate::schemas::schema_user::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewCustomer<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: &'a str,
}