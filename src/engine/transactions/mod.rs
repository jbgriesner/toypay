mod chargeback;
mod deposit;
mod dispute;
mod resolve;
mod withdrawal;

pub use chargeback::chargeback;
pub use deposit::deposit;
pub use dispute::dispute;
pub use resolve::resolve;
pub use withdrawal::withdrawal;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::storage::Storage;
    use crate::models::input_transaction::InputTransaction;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn test_storage() -> Storage {
        Storage::new()
    }

    fn input_transaction(
        transaction_type: &str,
        client: u16,
        tx: u32,
        amount: Option<&str>,
    ) -> InputTransaction {
        InputTransaction {
            transaction_type: transaction_type.to_string(),
            client,
            tx,
            amount: amount.map(|a| Decimal::from_str(a).unwrap()),
        }
    }

    mod deposit_tests {
        use super::*;

        #[test]
        fn test_deposit_successful() {
            let mut storage = test_storage();
            let tx = input_transaction("deposit", 1, 1, Some("10.50"));

            let result = deposit(&mut storage, tx);
            assert!(result.is_ok());

            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 1050);
            assert_eq!(account.held, 0);
            assert!(!account.locked);
        }

        #[test]
        fn test_deposit_no_amount() {
            let mut storage = test_storage();
            let tx = input_transaction("deposit", 1, 1, None);

            let result = deposit(&mut storage, tx);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Deposit requires amount"));
        }

        #[test]
        fn test_deposit_zero_amount() {
            let mut storage = test_storage();
            let tx = input_transaction("deposit", 1, 1, Some("0"));

            let result = deposit(&mut storage, tx);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Deposit amount must be positive"));
        }

        #[test]
        fn test_deposit_on_locked_account() {
            let mut storage = test_storage();

            let account = storage.get_account_mut(1);
            account.locked = true;

            let tx = input_transaction("deposit", 1, 1, Some("10.00"));
            let result = deposit(&mut storage, tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 0);
            assert!(account.locked);
        }
    }

    mod withdrawal_tests {
        use super::*;

        #[test]
        fn test_withdrawal_successful() {
            let mut storage = test_storage();

            let deposit_tx = input_transaction("deposit", 1, 1, Some("20.00"));
            deposit(&mut storage, deposit_tx).unwrap();

            let withdrawal_tx = input_transaction("withdrawal", 1, 2, Some("5.00"));
            let result = withdrawal(&mut storage, withdrawal_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 1500);
        }

        #[test]
        fn test_withdrawal_insufficient_funds() {
            let mut storage = test_storage();

            let deposit_tx = input_transaction("deposit", 1, 1, Some("5.00"));
            deposit(&mut storage, deposit_tx).unwrap();

            let withdrawal_tx = input_transaction("withdrawal", 1, 2, Some("10.00"));
            let result = withdrawal(&mut storage, withdrawal_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 500);
        }

        #[test]
        fn test_withdrawal_no_amount() {
            let mut storage = test_storage();
            let tx = input_transaction("withdrawal", 1, 1, None);

            let result = withdrawal(&mut storage, tx);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Withdrawal requires amount"));
        }

        #[test]
        fn test_withdrawal_on_locked_account() {
            let mut storage = test_storage();

            let deposit_tx = input_transaction("deposit", 1, 1, Some("20.00"));
            deposit(&mut storage, deposit_tx).unwrap();

            let account = storage.get_account_mut(1);
            account.locked = true;

            let withdrawal_tx = input_transaction("withdrawal", 1, 2, Some("5.00"));
            let result = withdrawal(&mut storage, withdrawal_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 2000);
            assert!(account.locked);
        }
    }

    mod dispute_tests {
        use super::*;

        fn setup_account_with_deposit(
            storage: &mut Storage,
            client: u16,
            tx_id: u32,
            amount: &str,
        ) {
            let deposit_tx = input_transaction("deposit", client, tx_id, Some(amount));
            deposit(storage, deposit_tx).unwrap();
        }

        #[test]
        fn test_dispute_successful() {
            let mut storage = test_storage();
            setup_account_with_deposit(&mut storage, 1, 1, "10.00");

            let dispute_tx = input_transaction("dispute", 1, 1, None);
            let result = dispute(&mut storage, dispute_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 0);
            assert_eq!(account.held, 1000);
            assert!(!account.locked);
        }

        #[test]
        fn test_dispute_nonexistent_transaction() {
            let mut storage = test_storage();
            setup_account_with_deposit(&mut storage, 1, 1, "10.00");

            let dispute_tx = input_transaction("dispute", 1, 999, None); // Non-existent tx
            let result = dispute(&mut storage, dispute_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 1000);
            assert_eq!(account.held, 0);
        }

        #[test]
        fn test_dispute_wrong_client() {
            let mut storage = test_storage();
            setup_account_with_deposit(&mut storage, 1, 1, "10.00");

            let dispute_tx = input_transaction("dispute", 2, 1, None); // Wrong client
            let result = dispute(&mut storage, dispute_tx);

            assert!(result.is_ok());
            let account1 = storage.get_account_mut(1);
            assert_eq!(account1.available, 1000);
            assert_eq!(account1.held, 0);
        }

        #[test]
        fn test_dispute_insufficient_available_funds() {
            let mut storage = test_storage();
            setup_account_with_deposit(&mut storage, 1, 1, "10.00");

            let withdrawal_tx = input_transaction("withdrawal", 1, 2, Some("8.00"));
            withdrawal(&mut storage, withdrawal_tx).unwrap();

            let dispute_tx = input_transaction("dispute", 1, 1, None);
            let result = dispute(&mut storage, dispute_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 200);
            assert_eq!(account.held, 0);
        }
    }

    mod resolve_tests {
        use super::*;

        fn setup_disputed_transaction(
            storage: &mut Storage,
            client: u16,
            tx_id: u32,
            amount: &str,
        ) {
            let deposit_tx = input_transaction("deposit", client, tx_id, Some(amount));
            deposit(storage, deposit_tx).unwrap();

            let dispute_tx = input_transaction("dispute", client, tx_id, None);
            dispute(storage, dispute_tx).unwrap();
        }

        #[test]
        fn test_resolve_successful() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let resolve_tx = input_transaction("resolve", 1, 1, None);
            let result = resolve(&mut storage, resolve_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 1000);
            assert_eq!(account.held, 0);
            assert!(!account.locked);
        }

        #[test]
        fn test_resolve_nonexistent_transaction() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let resolve_tx = input_transaction("resolve", 1, 999, None); // Non-existent
            let result = resolve(&mut storage, resolve_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 0);
            assert_eq!(account.held, 1000);
        }

        #[test]
        fn test_resolve_not_disputed() {
            let mut storage = test_storage();
            let deposit_tx = input_transaction("deposit", 1, 1, Some("10.00"));
            deposit(&mut storage, deposit_tx).unwrap();

            let resolve_tx = input_transaction("resolve", 1, 1, None);
            let result = resolve(&mut storage, resolve_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 1000);
            assert_eq!(account.held, 0);
        }

        #[test]
        fn test_resolve_wrong_client() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let resolve_tx = input_transaction("resolve", 2, 1, None); // Wrong client
            let result = resolve(&mut storage, resolve_tx);

            assert!(result.is_ok());
            let account1 = storage.get_account_mut(1);
            assert_eq!(account1.available, 0);
            assert_eq!(account1.held, 1000);
        }
    }

    mod chargeback_tests {
        use super::*;

        fn setup_disputed_transaction(
            storage: &mut Storage,
            client: u16,
            tx_id: u32,
            amount: &str,
        ) {
            let deposit_tx = input_transaction("deposit", client, tx_id, Some(amount));
            deposit(storage, deposit_tx).unwrap();

            let dispute_tx = input_transaction("dispute", client, tx_id, None);
            dispute(storage, dispute_tx).unwrap();
        }

        #[test]
        fn test_chargeback_successful() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let chargeback_tx = input_transaction("chargeback", 1, 1, None);
            let result = chargeback(&mut storage, chargeback_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 0);
            assert_eq!(account.held, 0);
            assert!(account.locked);
        }

        #[test]
        fn test_chargeback_nonexistent_transaction() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let chargeback_tx = input_transaction("chargeback", 1, 999, None); // Non-existent
            let result = chargeback(&mut storage, chargeback_tx);

            assert!(result.is_ok());
            let account = storage.get_account_mut(1);
            assert_eq!(account.available, 0);
            assert_eq!(account.held, 1000);
            assert!(!account.locked);
        }

        #[test]
        fn test_chargeback_wrong_client() {
            let mut storage = test_storage();
            setup_disputed_transaction(&mut storage, 1, 1, "10.00");

            let chargeback_tx = input_transaction("chargeback", 2, 1, None); // Wrong client
            let result = chargeback(&mut storage, chargeback_tx);

            assert!(result.is_ok());
            let account1 = storage.get_account_mut(1);
            assert_eq!(account1.available, 0);
            assert_eq!(account1.held, 1000);
            assert!(!account1.locked);
        }
    }
}
