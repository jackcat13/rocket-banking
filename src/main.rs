#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::{Build, Rocket};

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
    let _rocket = build_rocket().launch().await?;
    Ok(())
}

fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![banks])
}

#[cfg(test)]
mod test {
    use super::{build_rocket, rocket};
    use rocket::http::uncased::Uncased;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use std::borrow::Cow;

    #[test]
    fn index_should_return_hello_world() {
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn banks_should_return_mock_bank_account_details() {
        let valid_credentials =
            "{ \"basic\" : [{ \"identifier\": \"1\", \"password\": \"1\", \"bank\": \"Mock\" }] }";
        let expected_response = "[{\"label\":\"Mock bank account.\",\"balance\":1010.022}]";
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::banks))
            .header(Header {
                name: Uncased {
                    string: Cow::from("credentials"),
                },
                value: Cow::from(valid_credentials),
            })
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected_response);
    }

    #[test]
    fn banks_should_return_missing_header_error() {
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::banks)).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn banks_should_return_invalid_credentials_error_when_json_not_correct() {
        let invalid_credentials =
            "{ \"basic\" : [{ \"wrong\": \"1\", \"password\": \"1\", \"bank\": \"DoesNotExist\" }] }";
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::banks))
            .header(Header {
                name: Uncased {
                    string: Cow::from("credentials"),
                },
                value: Cow::from(invalid_credentials),
            })
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn banks_should_return_invalid_credentials_error_when_bank_not_supported() {
        let invalid_credentials =
            "{ \"basic\" : [{ \"identifier\": \"1\", \"password\": \"1\", \"bank\": \"DoesNotExist\" }] }";
        let client = Client::tracked(build_rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::banks))
            .header(Header {
                name: Uncased {
                    string: Cow::from("credentials"),
                },
                value: Cow::from(invalid_credentials),
            })
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }
}
