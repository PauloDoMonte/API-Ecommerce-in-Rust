use crate::schemas::schema_user::schema::users;
use crate::models::user::User;
use crate::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use diesel::prelude::*;
use rocket::get;

#[get("/")]
pub async fn get_users(db: &State<DbPool>) -> Result<Json<Vec<User>>, String> {
    let conn = db.get().map_err(|_| "Falha em conectar com a pool".to_string())?;

    let user_list: Vec<User> = users::table
        .load(&*conn)
        .map_err(|e| format!("Erro ao carregar os usuarios: {}", e))?;

    Ok(Json(user_list))
}