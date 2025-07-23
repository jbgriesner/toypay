use crate::engine::{storage::Storage, utils::DecimalToU32};
use crate::models::input_transaction::InputTransaction;
use anyhow::{anyhow, Result};

pub fn withdrawal(store: &mut Storage, tx: InputTransaction) -> Result<()> {
    let amount = tx
        .amount
        .ok_or_else(|| anyhow!("Withdrawal requires amount"))?;
    let amount_centimes = amount.decimal_to_u32()?;

    if amount_centimes == 0 {
        return Err(anyhow!("Withdrawal amount must be positive -> tx ignored"));
    }

    let account = store.get_account_mut(tx.client);

    if account.locked {
        return Ok(());
    }

    if account.available >= amount_centimes {
        account.available -= amount_centimes;
    }
    Ok(())
}
