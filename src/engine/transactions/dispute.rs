use crate::engine::storage::Storage;
use crate::models::input_transaction::InputTransaction;
use anyhow::Result;

pub fn dispute(store: &mut Storage, tx: InputTransaction) -> Result<()> {
    if let Some(original_tx) = store.get_transaction(tx.tx, tx.client) {
        if original_tx.client == tx.client && !original_tx.disputed {
            let account = store.get_account_mut(tx.client);

            if !account.locked && account.available >= original_tx.amount {
                account.available -= original_tx.amount;
                account.held += original_tx.amount;

                store.update_transaction_dispute(tx.tx, tx.client, true);
            }
        }
    }
    Ok(())
}
