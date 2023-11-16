use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Bank {
    pub label: String,
    pub balance: f64,
}
