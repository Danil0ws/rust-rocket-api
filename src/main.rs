#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod services;

use dotenv;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build().mount("/", routes![routes::index, routes::cep::find])
}
