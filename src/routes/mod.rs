pub mod cep;

#[get("/")]
pub fn index() -> &'static str {
    "Rust Rocket API"
}
