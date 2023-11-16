use crate::client::bank_client::BankClientDelegate;
use crate::model::bank::Bank;
use crate::model::credentials::Credential;

pub struct MockBankClient{}

impl BankClientDelegate for MockBankClient {
    fn get_bank_account(&self, _: &Credential) -> Bank {
        Bank {
            label: "Mock bank account.".to_string(),
            balance: 1010.022,
        }
    }
}