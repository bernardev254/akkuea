use crate::datatype::{Auction, AuctionStatus, Product, ProductCondition};
use super::storage::{get_and_increment_auction_counter, get_auction, is_verifier, save_auction, add_to_user_selling};
use soroban_sdk::xdr::ToXdr;

// Create a new auction
pub fn create_auction(
    env: &Env,
    seller: &Address,
    name: &String,
    description: &String,
    condition: &ProductCondition,
    images: &Vec<String>,
    inventory_count: &u32,
    reserve_price: &i128,
    start_time: &u64,
    end_time: &u64,
) -> BytesN<32> {
    seller.require_auth();

    // Validate inputs
    if start_time >= end_time {
        panic!("End time must be after start time");
    }
    if *inventory_count == 0 {
        panic!("Inventory count must be greater than 0");
    }
    if *reserve_price <= 0 {
        panic!("Reserve price must be greater than 0");
    }

    // Generate unique IDs by getting and incrementing the counter
    let counter = get_and_increment_auction_counter(env);

    // Create unique seed data for hashing
    let auction_seed = format!(
        "{}{}{}",
        counter,
        seller.to_string(),
        env.ledger().timestamp()
    );

    // Create auction ID using sha256
    let auction_id = env.crypto().sha256(&auction_seed.into_val(env)).into();

    // Create product ID using auction ID
    let product_seed = format!("{}product", auction_id.to_string());

    let product_id = env.crypto().sha256(&product_seed.into_val(env)).into();

    // Create product
    let product = Product {
        id: product_id,
        name: name.clone(),
        description: description.clone(),
        condition: *condition,
        images: images.clone(),
        seller: seller.clone(),
        inventory_count: *inventory_count,
        is_authenticated: false,
    };

    // Create auction
    let auction = Auction {
        id: auction_id,
        product,
        status: AuctionStatus::Pending,
        start_time: *start_time,
        end_time: *end_time,
        reserve_price: *reserve_price,
        current_highest_bid: None,
        all_bids: Vec::new(env),
        shipping: None,
        dispute_status: crate::datatype::DisputeStatus::None,
        dispute_reason: None,
    };

    // Save the auction
    save_auction(env, &auction_id, &auction);

    // Add to user's selling auctions
    add_to_user_selling(env, seller, &auction_id);

    // Emit event
    env.events().publish(
        (Symbol::new(env, "auction_created"), auction_id,
        auction_id,
    );

    auction_id,
}
