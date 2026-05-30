#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    token::Client as TokenClient,
    Address, Env, String,
};

// ── Storage keys ──────────────────────────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Auction,
    HighestBidder,
    HighestBid,
    Seller,
    Token,
    Deadline,
    Finalized,
    Cancelled,
    MinBid,
}

// ── Auction struct ────────────────────────────────────────────────────────────
#[contracttype]
pub struct AuctionInfo {
    pub seller:         Address,
    pub highest_bidder: Option<Address>,
    pub highest_bid:    i128,
    pub deadline:       u64,
    pub finalized:      bool,
    pub cancelled:      bool,
    pub min_bid:        i128,
    pub token:          Address,
}

// ── Contract ──────────────────────────────────────────────────────────────────
#[contract]
pub struct NoLossAuction;

#[contractimpl]
impl NoLossAuction {

    // Create a new auction
    pub fn create_auction(
        env: Env,
        seller: Address,
        token: Address,
        min_bid: i128,
        deadline: u64,
    ) {
        seller.require_auth();

        let auction = AuctionInfo {
            seller:         seller.clone(),
            highest_bidder: None,
            highest_bid:    0,
            deadline,
            finalized:      false,
            cancelled:      false,
            min_bid,
            token,
        };

        env.storage().persistent().set(&DataKey::Auction, &auction);
    }

    // Place a bid
    pub fn place_bid(env: Env, bidder: Address, amount: i128) {
        bidder.require_auth();

        let mut auction: AuctionInfo = env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap();

        // check auction is still open
        if auction.finalized {
            panic!("auction already finalized");
        }
        if auction.cancelled {
            panic!("auction was cancelled");
        }
        if env.ledger().timestamp() > auction.deadline {
            panic!("auction deadline has passed");
        }
        if amount <= auction.highest_bid {
            panic!("bid must be higher than current highest bid");
        }
        if amount < auction.min_bid {
            panic!("bid is below minimum bid");
        }

        let token = TokenClient::new(&env, &auction.token);

        // refund previous highest bidder automatically
        if let Some(prev_bidder) = auction.highest_bidder.clone() {
            token.transfer(
                &env.current_contract_address(),
                &prev_bidder,
                &auction.highest_bid,
            );
        }

        // collect new bid from bidder
        token.transfer_from(
            &env.current_contract_address(),
            &bidder,
            &env.current_contract_address(),
            &amount,
        );

        // update auction state
        auction.highest_bidder = Some(bidder);
        auction.highest_bid    = amount;

        env.storage().persistent().set(&DataKey::Auction, &auction);
    }

    // Finalize auction after deadline — send funds to seller
    pub fn finalize_auction(env: Env) {
        let mut auction: AuctionInfo = env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap();

        if auction.finalized {
            panic!("already finalized");
        }
        if auction.cancelled {
            panic!("auction was cancelled");
        }
        if env.ledger().timestamp() <= auction.deadline {
            panic!("auction deadline has not passed yet");
        }

        auction.finalized = true;
        env.storage().persistent().set(&DataKey::Auction, &auction);

        // transfer winning bid to seller
        if let Some(_winner) = auction.highest_bidder.clone() {
            let token = TokenClient::new(&env, &auction.token);
            token.transfer(
                &env.current_contract_address(),
                &auction.seller,
                &auction.highest_bid,
            );
        }
    }

    // Cancel auction — only if no bids exist
    pub fn cancel_auction(env: Env, seller: Address) {
        seller.require_auth();

        let mut auction: AuctionInfo = env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap();

        if auction.finalized {
            panic!("auction already finalized");
        }
        if auction.highest_bid > 0 {
            panic!("cannot cancel — bids already exist");
        }

        auction.cancelled = true;
        env.storage().persistent().set(&DataKey::Auction, &auction);
    }

    // Get current auction info
    pub fn get_auction(env: Env) -> AuctionInfo {
        env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap()
    }

    // Get highest bidder
    pub fn get_highest_bidder(env: Env) -> Option<Address> {
        let auction: AuctionInfo = env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap();
        auction.highest_bidder
    }

    // Get highest bid
    pub fn get_highest_bid(env: Env) -> i128 {
        let auction: AuctionInfo = env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .unwrap();
        auction.highest_bid
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Env, Address,
    };

    fn setup() -> (Env, NoLossAuctionClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(NoLossAuction, ());
        let client      = NoLossAuctionClient::new(&env, &contract_id);
        let seller      = Address::generate(&env);
        let bidder1     = Address::generate(&env);
        let token       = Address::generate(&env);

        // set deadline 1000 seconds from now
        env.ledger().with_mut(|l| l.timestamp = 1000);

        client.create_auction(&seller, &token, &100, &2000);

        (env, client, seller, bidder1, token)
    }

    #[test]
    fn test_create_auction() {
        let (_, client, seller, _, token) = setup();
        let auction = client.get_auction();
        assert_eq!(auction.seller, seller);
        assert_eq!(auction.min_bid, 100);
        assert_eq!(auction.highest_bid, 0);
        assert!(!auction.finalized);
        assert!(!auction.cancelled);
    }

    #[test]
    fn test_cancel_no_bids() {
        let (_, client, seller, _, _) = setup();
        client.cancel_auction(&seller);
        let auction = client.get_auction();
        assert!(auction.cancelled);
    }
}