use rocket::serde::{Serialize, json::Json};

mod model;

#[macro_use] extern crate rocket;

use crate::model::bank::Bank;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/banks")]
async fn banks() -> Result<Json<Vec<Bank>>, ()> {
    Ok(Json(vec![Bank{
        label: "A bank".to_string(),
        balance: 100.12
    }]))
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