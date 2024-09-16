// routes/register.rs

use rocket::get;
use rocket::{post, serde::json::Json, State};
use diesel::prelude::*; // Importa QueryDsl e outros traits necessários
use crate::models::customer_management::customer::{Customer, NewCustomer};
use crate::schemas::schema_user::schema::users;
use crate::DbPool;

#[post("/", format = "json", data = "<Customer>")]
pub async fn register_post(user: Json<NewCustomer<'_>>, db: &State<DbPool>) -> Result<Json<Customer>, String> {
    let conn = db.get().map_err(|_| "Failed to get a connection from the pool".to_string())?;

    // Verifica se já existe um usuário com o mesmo email
    let existing_email = users::table
        .filter(users::email.eq(&user.email))
        .first::<Customer>(&*conn)
        .optional()
        .map_err(|e| format!("Error checking for existing email: {}", e))?;

    if existing_email.is_some() {
        return Err("Email already exists".to_string());
    }

    // Verifica se já existe um usuário com o mesmo username
    let existing_username = users::table
        .filter(users::username.eq(&user.username))
        .first::<Customer>(&*conn)
        .optional()
        .map_err(|e| format!("Error checking for existing username: {}", e))?;

    if existing_username.is_some() {
        return Err("Username already exists".to_string());
    }

    let new_user = NewCustomer {
        email: &user.email,
        username: &user.username,
        password_hash: &user.password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&*conn)
        .map_err(|_| "Error inserting new user".to_string())?;

    let inserted_user: User = users::table
        .order(users::id.desc())
        .first(&*conn)
        .map_err(|e| format!("Error retrieving inserted user: {}", e))?;

    println!("User registered: {:?}", inserted_user);

    Ok(Json(inserted_user))
}

#[get("/")]
pub fn register_get() -> &'static str {
    "Registre-se"
}