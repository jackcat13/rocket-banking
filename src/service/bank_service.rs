use crate::client::bank_client::BankClientDelegate;
use crate::client::mock_bank_client::MockBankClient;
use rocket::serde::json::Json;

use crate::model::bank::Bank;
use crate::model::bank_enum::BankEnum;
use crate::model::credentials::{Credential, Credentials};

pub async fn banks_aggregate(credentials: &Credentials) -> Result<Json<Vec<Bank>>, ()> {
    let aggregation = credentials
        .basic
        .iter()
        .map(|credential| query_bank(resolve_client_delegate(&credential.bank), credential))
        .collect();
    Ok(Json(aggregation))
}

fn resolve_client_delegate(bank: &BankEnum) -> &MockBankClient {
    match bank {
        BankEnum::Mock => &MockBankClient {},
    }
}

fn query_bank(client_delegate: &dyn BankClientDelegate, credential: &Credential) -> Bank {
    client_delegate.get_bank_account(credential)
}
