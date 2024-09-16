use rocket::get;

#[get("/")]
pub fn product() -> &'static str {
    "Produtos"
}
