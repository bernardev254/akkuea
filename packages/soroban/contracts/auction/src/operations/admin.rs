use super::storage::get_admin;
use crate::datatype::{Auction, StorageKey};
use soroban_sdk::{Address, BytesN, Env, Map, Vec};

// Initialize the contract with an admin
pub fn initialize(env: &Env, admin: &Address) {
    if env.storage().instance().has(&StorageKey::Admin) {
        panic!("Contract already initialized");
    }

    admin.require_auth();
    env.storage().instance().set(&StorageKey::Admin, admin);
    env.storage()
        .instance()
        .set(&StorageKey::AuctionCounter, &0u32);

    // Create empty maps and vectors with proper type annotations
    let auctions_map: Map<BytesN<32>, Auction> = Map::new(env);
    env.storage()
        .instance()
        .set(&StorageKey::Auctions, &auctions_map);

    env.storage()
        .instance()
        .set(&StorageKey::Auctions, &Map::<BytesN<32>, Auction>::new(env));

    env.storage()
        .instance()
        .set(&StorageKey::Resolvers, &Vec::<Address>::new(env));
}
