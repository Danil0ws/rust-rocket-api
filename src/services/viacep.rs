use reqwest::Error;
use serde_json::Value;

pub use crate::models::viacep::ViaCep;
use async_trait::async_trait;


macro_rules! URL {
    () => { "https://viacep.com.br/ws/{}/json/" };
}

#[async_trait]
pub trait Ceps<T> {
    fn instance() -> Self;
    async fn request(&mut self, params: String) -> Result<T, Error>;
}

#[async_trait]
impl Ceps<String> for ViaCep {
    fn instance() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }
    async fn request(&mut self, params: String) -> Result<String, Error> {
        let ip = self.client.get(format!(URL!(), params.to_string())).send().await?.text().await;
        return ip
    }    
}

pub struct CepFactory {}

impl CepFactory {
    pub async fn get(cep: &str) -> Value {
        let request = <ViaCep>::instance().request(cep.to_string()).await;

        match request {
            Ok(date) => {
                let res: Value = serde_json::json!(&date);
                res
            },
            Err(e) => panic!("{}", e),
        }
    }    
}