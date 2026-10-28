# No-Loss Auction Protocol

An experimental auction smart contract built with Rust and Soroban on Stellar. Bidders compete using a specified token; a new highest bid refunds the previous highest bidder, and the winning bid can be sent to the seller after the auction deadline.

> **Status:** Educational prototype. The current automated tests cover auction creation and cancellation without bids only. This project has not been represented as security-audited or production-ready.

## Features in the current contract

- Create an auction with a seller, token contract, minimum bid, and deadline.
- Record a bid that exceeds both the current highest bid and the minimum bid.
- Refund the previous highest bidder when a higher bid is accepted.
- Store the current highest bidder and bid amount.
- Finalize after the deadline and transfer the winning bid to the seller.
- Cancel an auction when no bid has been placed.
- Read auction state, highest bidder, and highest bid.

## Technology

- Rust
- Soroban SDK
- Stellar Testnet tooling
- HTML / JavaScript interface files

## Contract interface

| Function | Purpose |
|---|---|
| `create_auction(seller, token, min_bid, deadline)` | Creates the auction and records its configuration. |
| `place_bid(bidder, amount)` | Accepts a higher eligible bid and refunds the prior highest bidder. |
| `finalize_auction()` | Finalizes after the deadline and transfers the winning bid to the seller. |
| `cancel_auction(seller)` | Cancels an auction with no bids. |
| `get_auction()` | Returns the stored auction state. |
| `get_highest_bidder()` | Returns the highest bidder, if any. |
| `get_highest_bid()` | Returns the current highest bid. |

## Repository structure

```text
.
├── Cargo.toml
├── contracts/
│   └── no_loss_auction/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   └── test.rs
│       └── src/index.html
├── index.html
└── README.md
```

## Build and test

With Rust and the Stellar CLI / Soroban tooling installed:

```bash
cargo test
cargo build --target wasm32v1-none --release
```

For deployment, use the Stellar CLI version and network configuration appropriate to your environment. Do not expose secret keys in source code or commit them to this repository.

## Current contract ID

The repository currently lists this Stellar Testnet contract ID:

```text
CA5Z7VCHKYAYDI7AY7IABRRZQH7VAVVEEEXHEYTKYNSCGVFA6IQUJUS2
```

Verify that the deployment is still available and matches the current source before relying on it.

## Areas to strengthen

Before treating this as more than a prototype, add validation and tests for repeated initialization/auction creation, invalid or negative amounts, bidding at deadline boundaries, finalization with and without bids, cancellation state transitions, token-transfer failures, and authorization. The current tests are not sufficient to establish security.
