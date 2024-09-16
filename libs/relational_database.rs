extern crate diesel;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env

    // Obter a URL do banco de dados a partir das variáveis de ambiente
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Estabelecer o pool de conexões
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}