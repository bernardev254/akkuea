// Export all operations
mod admin;
mod auction;
mod bid;
mod dispute;
mod query;
mod shipping;
mod storage;

// Re-export all functions to avoid exposing module structure details
pub use admin::{add_resolver, add_verifier, initialize};
pub use auction::{cancel_auction, create_auction, end_auction, start_auction, verify_product};
pub use bid::place_bid;
pub use dispute::{open_dispute, resolve_dispute};
pub use query::{
    calculate_shipping_cost, query_auction, query_auctions, query_user_bidding_auctions,
    query_user_selling_auctions,
};
pub use shipping::{add_shipping_info, update_shipping_status};
