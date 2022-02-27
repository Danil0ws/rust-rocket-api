#![feature(proc_macro_hygiene)] 
#![feature(decl_macro)]

extern crate serde;
#[macro_use]
extern crate serde_json;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thing {
    pub name: String,
}
#[derive(Debug)]
struct ApiResponse<T> {
    json: Json<T>,
    status: Status,
}

impl<'r, T: serde::Serialize> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
