use crate::model::bank_enum::BankEnum;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub basic: Vec<Credential>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub(crate) identifier: String,
    pub(crate) password: String,
    pub bank: BankEnum,
}

const CREDENTIALS_HEADER: &str = "credentials";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Credentials {
    type Error = CredentialsError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        if headers.get(CREDENTIALS_HEADER).count() != 1 {
            return Outcome::Error((Status::BadRequest, CredentialsError::Missing));
        }
        let credentials = serde_json::from_str(headers.get_one(CREDENTIALS_HEADER).unwrap());
        match credentials {
            Ok(credentials) => Outcome::Success(credentials),
            Err(_) => Outcome::Error((Status::BadRequest, CredentialsError::Invalid)),
        }
    }
}

#[derive(Debug)]
pub enum CredentialsError {
    Missing,
    Invalid,
}
