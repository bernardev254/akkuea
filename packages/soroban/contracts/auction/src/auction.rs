#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    vec, Env,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuctionContract);
    let client = AuctionContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Initialize the contract
    let result = client.initialize(&admin);
    assert!(result.is_ok());

    // Try to initialize again - should fail
    let result = client.initialize(&admin);
    assert!(result.is_err());
}

#[test]
fn test_create_and_manage_auction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuctionContract);
    let client = AuctionContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let bidder1 = Address::generate(&env);
    let bidder2 = Address::generate(&env);

    // Initialize the contract
    client.initialize(&admin).unwrap();

    // Create auction
    let name = String::from_str(&env, "Vintage Textbook");
    let description = String::from_str(&env, "A rare educational textbook from 1950");
    let condition = ProductCondition::Good;
    let mut images = Vec::new(&env);
    images.push_back(String::from_str(&env, "ipfs://image1.jpg"));
    images.push_back(String::from_str(&env, "ipfs://image2.jpg"));

    let start_time = env.ledger().timestamp() + 100;
    let end_time = start_time + 3600; // 1 hour auction

    let auction_id = client
        .create_auction(
            &seller,
            &name,
            &description,
            &condition,
            &images,
            &2,    // 2 items available
            &1000, // reserve price
            &start_time,
            &end_time,
        )
        .unwrap();

    // Check auction was created correctly
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.status, AuctionStatus::Pending);
    assert_eq!(auction.product.name, name);
    assert_eq!(auction.product.inventory_count, 2);

    // Can't place bids before auction starts
    let result = client.place_bid(&auction_id, &bidder1, &1500, &1);
    assert!(result.is_err());

    // Start the auction
    env.ledger().set_timestamp(start_time + 10);
    client.start_auction(&auction_id).unwrap();

    // Now we can place bids
    client.place_bid(&auction_id, &bidder1, &1500, &1).unwrap();

    // Bidder2 places higher bid
    client.place_bid(&auction_id, &bidder2, &2000, &1).unwrap();

    // Check that bidder2 is now highest
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.current_highest_bid.unwrap().bidder, bidder2);

    // End the auction
    env.ledger().set_timestamp(end_time + 10);
    client.end_auction(&auction_id).unwrap();

    // Add shipping info
    let tracking = String::from_str(&env, "TRK123456789");
    let carrier = String::from_str(&env, "Educational Express");
    let est_delivery = end_time + 86400; // 1 day after auction end
    let shipping_cost = 500; // 5.00 units
    let recipient = String::from_str(&env, "123 Learner Ave, Knowledge City");

    client
        .add_shipping_info(
            &auction_id,
            &tracking,
            &carrier,
            &est_delivery,
            &shipping_cost,
            &recipient,
        )
        .unwrap();

    // Update shipping status
    client
        .update_shipping_status(&auction_id, &ShippingStatus::InTransit)
        .unwrap();

    // Verify everything was updated correctly
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.status, AuctionStatus::Ended);
    assert_eq!(auction.shipping.unwrap().status, ShippingStatus::InTransit);

    // Mark as delivered
    client
        .update_shipping_status(&auction_id, &ShippingStatus::Delivered)
        .unwrap();

    // Check auction completed
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.status, AuctionStatus::Completed);
}

#[test]
fn test_dispute_resolution() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuctionContract);
    let client = AuctionContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let resolver = Address::generate(&env);

    // Initialize the contract
    client.initialize(&admin).unwrap();

    // Add resolver
    client.add_resolver(&admin, &resolver).unwrap();

    // Create and run through an auction
    let name = String::from_str(&env, "Educational Software");
    let description = String::from_str(&env, "Programming tutorial software");
    let condition = ProductCondition::New;
    let images = Vec::new(&env);

    let start_time = env.ledger().timestamp() + 100;
    let end_time = start_time + 3600;

    let auction_id = client
        .create_auction(
            &seller,
            &name,
            &description,
            &condition,
            &images,
            &1,
            &1000,
            &start_time,
            &end_time,
        )
        .unwrap();

    // Start auction and place bid
    env.ledger().set_timestamp(start_time + 10);
    client.start_auction(&auction_id).unwrap();
    client.place_bid(&auction_id, &buyer, &1500, &1).unwrap();

    // End auction
    env.ledger().set_timestamp(end_time + 10);
    client.end_auction(&auction_id).unwrap();

    // Buyer opens dispute
    let reason = String::from_str(&env, "Item not as described");
    client.open_dispute(&auction_id, &buyer, &reason).unwrap();

    // Check dispute status
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.dispute_status, DisputeStatus::Open);
    assert_eq!(auction.status, AuctionStatus::Disputed);

    // Resolver resolves in favor of buyer
    client
        .resolve_dispute(&resolver, &auction_id, &DisputeStatus::ResolvedForBuyer)
        .unwrap();

    // Check final status
    let auction = client.get_auction(&auction_id).unwrap();
    assert_eq!(auction.dispute_status, DisputeStatus::ResolvedForBuyer);
    assert_eq!(auction.status, AuctionStatus::Completed);
}
