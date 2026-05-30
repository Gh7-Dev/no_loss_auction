# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


# No-Loss Auction Protocol

A decentralized no-loss auction system built on Stellar using Soroban smart contracts.

## Contract ID (Testnet)
CA5Z7VCHKYAYDI7AY7IABRRZQH7VAVVEEEXHEYTKYNSCGVFA6IQUJUS2

## Features
- Create auction with deadline and minimum bid
- Place bids using SEP-41 tokens — previous highest bidder refunded automatically
- Track highest bidder and bid amount
- Finalize auction after deadline — transfers winning bid to seller
- Cancel auction only if no bids exist

## Functions
- `create_auction` — seller creates auction
- `place_bid` — bidder places bid, previous bidder refunded
- `finalize_auction` — sends winning bid to seller after deadline
- `cancel_auction` — cancels if no bids exist
- `get_auction` — view current auction state

## Tech Stack
- Smart Contract: Rust + Soroban SDK v26
- Frontend: HTML / JavaScript
- Network: Stellar Testnet

## Deploy
```bash
cargo build --target wasm32v1-none --release
stellar contract deploy --wasm target/wasm32v1-none/release/no_loss_auction.wasm --source my-key --network testnet
```