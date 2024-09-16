use crate::schemas::schema_user::schema::users;
use crate::models::customer_management::customer::{Customer, NewCustomer};
use crate::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use diesel::prelude::*;
use rocket::get;

#[get("/")]
pub async fn get_users(db: &State<DbPool>) -> Result<Json<Vec<Customer>>, String> {
    let conn = db.get().map_err(|_| "Falha em conectar com a pool".to_string())?;

    let user_list: Vec<Customer> = users::table
        .load(&*conn)
        .map_err(|e| format!("Erro ao carregar os usuarios: {}", e))?;

    Ok(Json(user_list))
}