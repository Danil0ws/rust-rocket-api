use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Deserialize, Serialize)]
pub struct CepInput {
    pub cep: String,
}