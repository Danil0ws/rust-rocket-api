// use crate::models::cep::CepInput;
// use rocket::serde::json::Json;

use rocket::{Response, http::ContentType};
use serde_json::json;

pub use crate::models::viacep::ViaCepResponse;

use crate::services::viacep::CepFactory;
#[get("/cep/<zip_code>")]
pub async fn find(zip_code: String) -> String {
    let create = CepFactory::get(&zip_code.to_string()).await; 
    let body = json!(create);
}