use rocket::get;

#[get("/")]
pub fn login() -> &'static str {
    "Faz login"
}
