# ToyPay

Welcome to ToyPay, probably the simplest payment engine in the world.

## Usage

Assuming all transactions are in a `transactions.csv` file, you can run ToyPay this way:

```bash
cargo run -- transactions.csv
```

## Bird View

ToyPay reads input transactions one by one, and processes them in isolated shards. The shards number is directly based on the number of CPU cores on local machine. When a transaction arrives, the first step is to dispatch the transaction in the "correct shard". The "correct shard" is just a modulo on the client id contained by the transaction. Then, depending on the transaction type, the transaction is pushed in a standard LRU cache. Each shard is associated with its own LRU cache. Each LRU cache contains up to 100k transactions. This allows to lookup for past transactions in O(1).

## "Limitations" (or Design Choices...)

- ToyPay is NOT multi-threaded, NEITHER distributed: the sharding strategy makes it super easy to make it multi-threaded in the future, if we want to, but this would be overkill at this stage
- Any error during any phase of a transaction process is simply ignored. I considered that more sophisticated error handling, logs or monitoring was definitely overkill as well here
- LRU caches imply that in case of a disputed transaction very old, the corresponding transaction could not be fetched. This is more a functional decision than a technical issue: I consider that a user cannot dispute a past transaction after an arbitrary timeout

## Tests

ToyPay comes along simple [unit tests](src/engine/transactions/mod.rs#L13) and [integration tests](tests/integration_tests.rs)

## AI Use

To comply with AI Policy, I declare that **GPT‑4o** helped me with:

- bootstrapping my sharding strategy, even if after some iterations with it I wasn't satisfied with the result, so at the end there is no real connection between what GPT provided and my final implementation
- writing tests: simple refactor was enough to make them useful
- writing samples data: I tried different scenarios to see how ToyPay performed
