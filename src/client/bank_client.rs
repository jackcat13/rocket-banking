use crate::model::bank::Bank;
use crate::model::credentials::Credential;

pub trait BankClientDelegate {
    fn get_bank_account(&self, credential: &Credential) -> Bank;
}