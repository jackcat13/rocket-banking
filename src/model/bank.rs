use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bank {
    pub label: String,
    pub balance: f64,
}