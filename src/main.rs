// main.rs

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod routes;
pub mod models;
pub mod repositories;
pub mod libs;
pub mod schemas;

use routes::home::home;
use routes::login::login;
use routes::register::{register_post, register_get};
use routes::product::product;
use routes::users::get_users;

use libs::relational_database::{establish_connection, DbPool};

#[launch]
fn rocket() -> _ {
    
    // Estabelecer o pool de conexões
    let db_pool = establish_connection();

    println!("Conectado com o banco de dados");

    rocket::build()
        .manage(db_pool) // Pool de conexão adicionada ao Rocket
        
        .configure(rocket::Config {
            port: 8081,
            ..Default::default()
        })

        .mount("/", routes![home])
        .mount("/produto", routes![product])
        .mount("/login", routes![login])
        .mount("/register", routes![register_post, register_get])
        .mount("/user", routes![get_users])


}
