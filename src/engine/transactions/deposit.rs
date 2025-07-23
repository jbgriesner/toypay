use crate::engine::storage::Storage;
use crate::engine::utils::DecimalToU32;
use crate::models::input_transaction::InputTransaction;
use crate::models::transaction::Transaction;
use anyhow::{anyhow, Result};

pub fn deposit(store: &mut Storage, tx: InputTransaction) -> Result<()> {
    let amount = tx
        .amount
        .ok_or_else(|| anyhow!("Deposit requires amount"))?;
    let amount_centimes = amount.decimal_to_u32()?;

    if amount_centimes == 0 {
        return Err(anyhow!("Deposit amount must be positive -> tx ignored"));
    }

    let account = store.get_account_mut(tx.client);

    if account.locked {
        return Ok(());
    }

    account.available = account.available.saturating_add(amount_centimes);

    let stored_tx = Transaction {
        client: tx.client,
        amount: amount_centimes,
        disputed: false,
    };

    store.store_transaction(tx.tx, stored_tx);

    Ok(())
}
