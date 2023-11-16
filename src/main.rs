#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

use crate::model::bank::Bank;
use crate::model::credentials::Credentials;
use crate::service::bank_service::banks_aggregate;

mod client;
mod model;
mod service;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/banks")]
async fn banks(credentials: Credentials) -> Result<Json<Vec<Bank>>, ()> {
    banks_aggregate(&credentials).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![banks])
        .launch()
        .await?;
    Ok(())
}
