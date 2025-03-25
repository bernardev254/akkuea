use soroban_sdk::contracttype;

/// Product condition rating
#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum ProductCondition {
    New,
    LikeNew,
    Good,
    Fair,
    Poor,
}

/// Auction status
#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum AuctionStatus {
    Pending,
    Active,
    Ended,
    Cancelled,
    Completed,
    Disputed,
}

/// Dispute status
#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum DisputeStatus {
    None,
    Open,
    ResolvedForBuyer,
    ResolvedForSeller,
}

/// Shipping status
#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum ShippingStatus {
    NotShipped,
    Shipped,
    InTransit,
    Delivered,
}

/// Storage keys for better organization
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Admin,
    Auctions,
    AuctionCounter,
    UserSelling(soroban_sdk::Address),
    UserBidding(soroban_sdk::Address),
    Verifiers,
    Resolvers,
}
