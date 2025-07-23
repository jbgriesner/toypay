use crate::{
    engine::sharding::Shards,
    models::{account::Account, output_record::OutputRecord, transaction::Transaction},
    num_cpus,
};
use lru::LruCache;
use std::{collections::HashMap, num::NonZeroUsize};

pub struct Storage {
    pub accounts: Shards<HashMap<u16, Account>>,
    pub transactions: Shards<LruCache<u32, Transaction>>,
}

impl Storage {
    pub fn new() -> Self {
        let num_shards = std::cmp::max(4, num_cpus::get_cpus());

        let accounts = (0..num_shards)
            .map(|_| HashMap::with_capacity(1000))
            .collect();
        let transactions = (0..num_shards)
            .map(|_| LruCache::new(NonZeroUsize::new(100000).unwrap()))
            .collect();

        Self {
            accounts: Shards::new(accounts),
            transactions: Shards::new(transactions),
        }
    }

    pub fn collect_accounts(&self) -> Vec<OutputRecord> {
        let mut all_accounts = Vec::new();

        for shard in self.accounts.shards_slices() {
            for (&client_id, &account) in shard {
                all_accounts.push(OutputRecord {
                    client: client_id,
                    available: account.available_as_decimal(),
                    held: account.held_as_decimal(),
                    total: account.total_as_decimal(),
                    locked: account.locked,
                });
            }
        }

        all_accounts.sort_by_key(|r| r.client);
        all_accounts
    }

    pub fn get_account_mut(&mut self, client_id: u16) -> &mut Account {
        self.accounts
            .get_shard(client_id)
            .entry(client_id)
            .or_insert_with(Account::new)
    }

    pub fn store_transaction(&mut self, tx_id: u32, tx: Transaction) {
        self.transactions.get_shard(tx.client).put(tx_id, tx);
    }

    pub fn get_transaction(&mut self, tx_id: u32, client_id: u16) -> Option<Transaction> {
        self.transactions.get_shard(client_id).get(&tx_id).copied()
    }

    pub fn update_transaction_dispute(&mut self, tx_id: u32, client_id: u16, disputed: bool) {
        if let Some(tx) = self.transactions.get_shard(client_id).get_mut(&tx_id) {
            tx.disputed = disputed;
        }
    }
}
