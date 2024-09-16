// schemas/schema_user.rs

pub mod schema {
    table! {
        users (id) {
            id -> Uuid,
            username -> Varchar,
            email -> Varchar,
            password_hash -> Varchar,
        }
    }
}