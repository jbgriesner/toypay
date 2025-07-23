use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct InputTransaction {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}
