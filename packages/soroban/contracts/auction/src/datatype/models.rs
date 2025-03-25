use super::enums::{AuctionStatus, DisputeStatus, ProductCondition, ShippingStatus};
use soroban_sdk::{contracttype, Address, BytesN, String, Vec};

/// Product information
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: BytesN<32>,
    pub name: String,
    pub description: String,
    pub condition: ProductCondition,
    pub images: Vec<String>,
    pub seller: Address,
    pub inventory_count: u32,
    pub is_authenticated: bool,
}

/// Bid information
#[contracttype]
#[derive(Clone)]
pub struct Bid {
    pub bidder: Address,
    pub amount: i128,
    pub timestamp: u64,
    pub quantity: u32,
}

/// Shipping information
#[contracttype]
#[derive(Clone)]
pub struct ShippingInfo {
    pub status: ShippingStatus,
    pub tracking_number: String,
    pub carrier: String,
    pub estimated_delivery: u64,
    pub shipping_cost: i128,
    pub recipient_address: String,
}

/// Auction data - modify to fix Option<T> issues
#[contracttype]
#[derive(Clone)]
pub struct Auction {
    pub id: BytesN<32>,
    pub product: Product,
    pub status: AuctionStatus,
    pub start_time: u64,
    pub end_time: u64,
    pub reserve_price: i128,
    pub current_highest_bid: Option<Bid>,
    pub all_bids: Vec<Bid>,
    pub shipping: Option<ShippingInfo>,
    pub dispute_status: DisputeStatus,
    pub dispute_reason: Option<String>,
}
