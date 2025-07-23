use anyhow::Result;
use rust_decimal::Decimal;
use std::str::FromStr;
use toypay::{models::input_transaction::InputTransaction, ToyEngine};

fn create_transaction(
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

fn dispatch_transaction(transactions: Vec<InputTransaction>) -> Result<ToyEngine> {
    let mut engine = ToyEngine::new();

    for transaction in transactions {
        engine.dispatch(transaction)?;
    }

    Ok(engine)
}

#[test]
fn test_complete_transaction_workflow() -> Result<()> {
    let transactions = vec![
        create_transaction("deposit", 1, 1, Some("100.00")),
        create_transaction("deposit", 2, 2, Some("50.00")),
        create_transaction("withdrawal", 1, 3, Some("25.00")),
        create_transaction("withdrawal", 2, 4, Some("10.00")),
        create_transaction("withdrawal", 1, 5, Some("200.00")),
        create_transaction("deposit", 1, 6, Some("30.00")),
        create_transaction("deposit", 2, 7, Some("20.00")),
    ];

    let engine = dispatch_transaction(transactions)?;
    let accounts = engine.get_all_accounts();

    assert_eq!(accounts.len(), 2);

    let client1 = accounts.iter().find(|a| a.client == 1).unwrap();
    assert_eq!(client1.available, Decimal::from_str("105.00")?);
    assert_eq!(client1.held, Decimal::ZERO);
    assert_eq!(client1.total, Decimal::from_str("105.00")?);
    assert!(!client1.locked);

    let client2 = accounts.iter().find(|a| a.client == 2).unwrap();
    assert_eq!(client2.available, Decimal::from_str("60.00")?);
    assert_eq!(client2.held, Decimal::ZERO);
    assert_eq!(client2.total, Decimal::from_str("60.00")?);
    assert!(!client2.locked);

    Ok(())
}

#[test]
fn test_dispute_resolution_and_chargeback_workflow() -> Result<()> {
    let transactions = vec![
        create_transaction("deposit", 1, 1, Some("100.00")),
        create_transaction("deposit", 2, 2, Some("75.00")),
        create_transaction("deposit", 3, 3, Some("50.00")),
        create_transaction("deposit", 1, 4, Some("25.00")),
        create_transaction("deposit", 2, 5, Some("30.00")),
        create_transaction("dispute", 1, 1, None),
        create_transaction("resolve", 1, 1, None),
        create_transaction("dispute", 2, 5, None),
        create_transaction("chargeback", 2, 5, None),
        create_transaction("dispute", 3, 999, None),
        create_transaction("deposit", 1, 6, Some("15.00")),
        create_transaction("deposit", 2, 7, Some("40.00")),
    ];

    let engine = dispatch_transaction(transactions)?;
    let accounts = engine.get_all_accounts();

    assert_eq!(accounts.len(), 3);

    let client1 = accounts.iter().find(|a| a.client == 1).unwrap();
    assert_eq!(client1.available, Decimal::from_str("140.00")?);
    assert_eq!(client1.held, Decimal::ZERO);
    assert_eq!(client1.total, Decimal::from_str("140.00")?);
    assert!(!client1.locked);

    let client2 = accounts.iter().find(|a| a.client == 2).unwrap();
    assert_eq!(client2.available, Decimal::from_str("75.00")?);
    assert_eq!(client2.held, Decimal::ZERO);
    assert_eq!(client2.total, Decimal::from_str("75.00")?);
    assert!(client2.locked);

    let client3 = accounts.iter().find(|a| a.client == 3).unwrap();
    assert_eq!(client3.available, Decimal::from_str("50.00")?);
    assert_eq!(client3.held, Decimal::ZERO);
    assert_eq!(client3.total, Decimal::from_str("50.00")?);
    assert!(!client3.locked);

    Ok(())
}
