use super::storage::{add_to_user_bidding, get_auction, save_auction};
use crate::datatype::{AuctionStatus, Bid};
use soroban_sdk::{Address, BytesN, Env, Symbol};

// Place a bid on an auction
pub fn place_bid(
    env: &Env,
    auction_id: &BytesN<32>,
    bidder: &Address,
    amount: &i128,
    quantity: &u32,
) {
    bidder.require_auth();

    let mut auction = get_auction(env, auction_id);

    // Validate auction state
    if auction.status != AuctionStatus::Active {
        panic!("Auction is not active");
    }

    let current_time = env.ledger().timestamp();
    if current_time < auction.start_time {
        panic!("Auction has not started yet");
    }
    if current_time > auction.end_time {
        panic!("Auction has already ended");
    }

    // Check inventory
    if *quantity > auction.product.inventory_count {
        panic!("Requested quantity exceeds available inventory");
    }

    // Check bid amount
    if let Some(highest_bid) = &auction.current_highest_bid {
        if amount <= &highest_bid.amount {
            panic!("Bid amount must be higher than current highest bid");
        }
    } else if *amount < auction.reserve_price {
        panic!("Bid amount must be at least the reserve price");
    }

    // Create new bid
    let new_bid = Bid {
        bidder: bidder.clone(),
        amount: *amount,
        timestamp: current_time,
        quantity: *quantity,
    };

    // Update auction
    auction.current_highest_bid = Some(new_bid.clone());
    auction.all_bids.push_back(new_bid);
    save_auction(env, auction_id, &auction);

    // Add to bidder's auctions
    add_to_user_bidding(env, bidder, auction_id);

    // Emit event
    env.events().publish(
        (Symbol::new(env, "bid_placed"), auction_id.clone()),
        (auction_id.clone(), bidder.clone(), *amount, *quantity),
    );
}
