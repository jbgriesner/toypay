use crate::{
    engine::{
        storage::Storage,
        transactions::{chargeback, deposit, dispute, resolve, withdrawal},
    },
    models::{input_transaction::InputTransaction, output_record::OutputRecord},
};
use anyhow::{anyhow, Result};

mod sharding;
mod storage;
mod transactions;
mod utils;

pub struct ToyEngine {
    store: Storage,
}

impl ToyEngine {
    pub fn new() -> Self {
        Self {
            store: Storage::new(),
        }
    }

    pub fn get_all_accounts(&self) -> Vec<OutputRecord> {
        self.store.collect_accounts()
    }

    pub fn dispatch(&mut self, tx: InputTransaction) -> Result<()> {
        let store = &mut self.store;
        match tx.transaction_type.as_str() {
            "deposit" => deposit(store, tx),
            "withdrawal" => withdrawal(store, tx),
            "dispute" => dispute(store, tx),
            "resolve" => resolve(store, tx),
            "chargeback" => chargeback(store, tx),
            _ => Err(anyhow!(
                "Unexpected transaction type: {}",
                tx.transaction_type
            )),
        }
    }
}
