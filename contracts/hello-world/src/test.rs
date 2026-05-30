#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Env, Address,
};

#[test]
fn test_create_auction() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(NoLossAuction, ());
    let client      = NoLossAuctionClient::new(&env, &contract_id);
    let seller      = Address::generate(&env);
    let token       = Address::generate(&env);

    env.ledger().with_mut(|l| l.timestamp = 1000);

    client.create_auction(&seller, &token, &100, &2000);

    let auction = client.get_auction();
    assert_eq!(auction.seller, seller);
    assert_eq!(auction.min_bid, 100);
    assert_eq!(auction.highest_bid, 0);
    assert!(!auction.finalized);
    assert!(!auction.cancelled);
}

#[test]
fn test_cancel_no_bids() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(NoLossAuction, ());
    let client      = NoLossAuctionClient::new(&env, &contract_id);
    let seller      = Address::generate(&env);
    let token       = Address::generate(&env);

    env.ledger().with_mut(|l| l.timestamp = 1000);

    client.create_auction(&seller, &token, &100, &2000);
    client.cancel_auction(&seller);

    let auction = client.get_auction();
    assert!(auction.cancelled);
}