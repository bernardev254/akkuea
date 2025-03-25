use soroban_sdk::{
    contracttype, Address, BytesN, Env, IntoVal, Map, String, Symbol, TryFromVal, Vec,
};

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
    Pending,   // Created but not yet started
    Active,    // Auction is live and accepting bids
    Ended,     // Auction time has expired
    Cancelled, // Auction was cancelled
    Completed, // Product has been delivered and verified
    Disputed,  // There is an active dispute on this auction
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

/// Product information
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: BytesN<32>,
    pub name: String,
    pub description: String,
    pub condition: ProductCondition,
    pub images: Vec<String>, // URLs or IPFS hashes to product images
    pub seller: Address,
    pub inventory_count: u32,   // Number of items available
    pub is_authenticated: bool, // Verification status
}

/// Bid information
#[contracttype]
#[derive(Clone)]
pub struct Bid {
    pub bidder: Address,
    pub amount: i128,
    pub timestamp: u64,
    pub quantity: u32, // For bulk bidding
}

/// Shipping information
#[contracttype]
#[derive(Clone)]
pub struct ShippingInfo {
    pub status: ShippingStatus,
    pub tracking_number: String,
    pub carrier: String,
    pub estimated_delivery: u64, // Timestamp
    pub shipping_cost: i128,
    pub recipient_address: String,
}

/// Auction data
#[contracttype]
#[derive(Clone)]
pub struct Auction {
    pub id: BytesN<32>,
    pub product: Product,
    pub status: AuctionStatus,
    pub start_time: u64,
    pub end_time: u64,
    pub reserve_price: i128, // Minimum price to be met
    pub current_highest_bid: Option<Bid>,
    pub all_bids: Vec<Bid>,
    pub shipping: Option<ShippingInfo>,
    pub dispute_status: DisputeStatus,
    pub dispute_reason: Option<String>,
}

/// Events emitted by the contract
#[contracttype]
#[derive(Clone)]
pub enum AuctionEvent {
    AuctionCreated(BytesN<32>),
    AuctionStarted(BytesN<32>),
    AuctionEnded(BytesN<32>),
    BidPlaced(BytesN<32>, Address, i128, u32),
    ProductShipped(BytesN<32>, String),
    ProductDelivered(BytesN<32>),
    DisputeOpened(BytesN<32>, String),
    DisputeResolved(BytesN<32>, DisputeStatus),
}

/// Contract data storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,                        // Contract admin address
    Auctions,                     // Map of all auctions
    UserSellingAuctions(Address), // Auctions where user is seller
    UserBiddingAuctions(Address), // Auctions where user has placed bids
    AuctionCounter,               // Counter for generating auction IDs
    ProductVerifiers,             // Addresses that can verify product authenticity
    DisputeResolvers,             // Addresses that can resolve disputes
}

// Initialize the contract with an admin
pub fn initialize(env: Env, admin: Address) -> Result<(), String> {
    if env.storage().instance().has(&DataKey::Admin) {
        return Err("Contract already initialized".into());
    }

    admin.require_auth();

    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage()
        .instance()
        .set(&DataKey::AuctionCounter, &0u32);
    env.storage()
        .instance()
        .set(&DataKey::Auctions, &Map::<BytesN<32>, Auction>::new(&env));
    env.storage()
        .instance()
        .set(&DataKey::ProductVerifiers, &Vec::<Address>::new(&env));
    env.storage()
        .instance()
        .set(&DataKey::DisputeResolvers, &Vec::<Address>::new(&env));

    Ok(())
}

// Create a new auction for a product
pub fn create_auction(
    env: Env,
    seller: Address,
    name: String,
    description: String,
    condition: ProductCondition,
    images: Vec<String>,
    inventory_count: u32,
    reserve_price: i128,
    start_time: u64,
    end_time: u64,
) -> Result<BytesN<32>, String> {
    seller.require_auth();

    if start_time >= end_time {
        return Err("End time must be after start time".into());
    }

    if inventory_count == 0 {
        return Err("Inventory count must be greater than 0".into());
    }

    if reserve_price <= 0 {
        return Err("Reserve price must be greater than 0".into());
    }

    // Generate a unique auction ID and product ID
    let counter = env
        .storage()
        .instance()
        .get::<_, u32>(&DataKey::AuctionCounter)
        .unwrap_or(0);
    let id_seed = (counter, seller.clone(), env.ledger().timestamp());
    let auction_id = env.crypto().sha256(&id_seed.into_val(&env));
    let product_id = env.crypto().sha256(&(auction_id, "product").into_val(&env));

    // Create product
    let product = Product {
        id: product_id,
        name,
        description,
        condition,
        images,
        seller: seller.clone(),
        inventory_count,
        is_authenticated: false, // Products start unauthenticated
    };

    // Create auction
    let auction = Auction {
        id: auction_id.clone(),
        product,
        status: AuctionStatus::Pending,
        start_time,
        end_time,
        reserve_price,
        current_highest_bid: None,
        all_bids: Vec::new(&env),
        shipping: None,
        dispute_status: DisputeStatus::None,
        dispute_reason: None,
    };

    // Store auction
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Update user auctions
    let user_key = DataKey::UserSellingAuctions(seller.clone());
    let mut user_auctions = env
        .storage()
        .instance()
        .get::<_, Vec<BytesN<32>>>(&user_key)
        .unwrap_or(Vec::new(&env));
    user_auctions.push_back(auction_id.clone());
    env.storage().instance().set(&user_key, &user_auctions);

    // Update counter
    env.storage()
        .instance()
        .set(&DataKey::AuctionCounter, &(counter + 1));

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "auction_created"), auction_id.clone()),
        auction_id.clone(),
    );

    Ok(auction_id)
}

// Start an auction (transition from Pending to Active)
pub fn start_auction(env: Env, auction_id: BytesN<32>) -> Result<(), String> {
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Only the seller can start the auction
    auction.product.seller.require_auth();

    if auction.status != AuctionStatus::Pending {
        return Err("Auction is not in Pending status".into());
    }

    let current_time = env.ledger().timestamp();
    if current_time < auction.start_time {
        return Err("Cannot start auction before the scheduled start time".into());
    }

    // Update status
    auction.status = AuctionStatus::Active;
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "auction_started"), auction_id.clone()),
        auction_id,
    );

    Ok(())
}

// Place a bid on an auction
pub fn place_bid(
    env: Env,
    auction_id: BytesN<32>,
    bidder: Address,
    amount: i128,
    quantity: u32,
) -> Result<(), String> {
    bidder.require_auth();

    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Check auction status
    if auction.status != AuctionStatus::Active {
        return Err("Auction is not active".into());
    }

    // Check auction time
    let current_time = env.ledger().timestamp();
    if current_time < auction.start_time {
        return Err("Auction has not started yet".into());
    }
    if current_time > auction.end_time {
        return Err("Auction has already ended".into());
    }

    // Check inventory
    if quantity > auction.product.inventory_count {
        return Err("Requested quantity exceeds available inventory".into());
    }

    // Check bid amount (must be higher than current highest)
    if let Some(highest_bid) = &auction.current_highest_bid {
        if amount <= highest_bid.amount {
            return Err("Bid amount must be higher than current highest bid".into());
        }
    } else if amount < auction.reserve_price {
        return Err("Bid amount must be at least the reserve price".into());
    }

    // Create new bid
    let new_bid = Bid {
        bidder: bidder.clone(),
        amount,
        timestamp: current_time,
        quantity,
    };

    // Update auction
    auction.current_highest_bid = Some(new_bid.clone());
    auction.all_bids.push_back(new_bid);
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Update user bidding auctions
    let user_key = DataKey::UserBiddingAuctions(bidder.clone());
    let mut user_auctions = env
        .storage()
        .instance()
        .get::<_, Vec<BytesN<32>>>(&user_key)
        .unwrap_or(Vec::new(&env));
    if !user_auctions.contains(&auction_id) {
        user_auctions.push_back(auction_id.clone());
        env.storage().instance().set(&user_key, &user_auctions);
    }

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "bid_placed"), auction_id.clone()),
        (auction_id, bidder, amount, quantity),
    );

    Ok(())
}

// End an auction (can be called by anyone after end_time)
pub fn end_auction(env: Env, auction_id: BytesN<32>) -> Result<(), String> {
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    if auction.status != AuctionStatus::Active {
        return Err("Auction is not active".into());
    }

    let current_time = env.ledger().timestamp();
    if current_time <= auction.end_time {
        return Err("Auction end time has not been reached".into());
    }

    // Update status
    auction.status = AuctionStatus::Ended;
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "auction_ended"), auction_id.clone()),
        auction_id,
    );

    Ok(())
}

// Cancel an auction (only possible in Pending status)
pub fn cancel_auction(env: Env, auction_id: BytesN<32>) -> Result<(), String> {
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Only the seller can cancel the auction
    auction.product.seller.require_auth();

    if auction.status != AuctionStatus::Pending {
        return Err("Only pending auctions can be cancelled".into());
    }

    // Update status
    auction.status = AuctionStatus::Cancelled;
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "auction_cancelled"), auction_id.clone()),
        auction_id,
    );

    Ok(())
}

// Verify product authenticity (only verifiers can do this)
pub fn verify_product(
    env: Env,
    verifier: Address,
    auction_id: BytesN<32>,
    is_authentic: bool,
) -> Result<(), String> {
    verifier.require_auth();

    // Check if verifier is authorized
    let verifiers = env
        .storage()
        .instance()
        .get::<_, Vec<Address>>(&DataKey::ProductVerifiers)
        .unwrap();
    if !verifiers.contains(&verifier) {
        return Err("Not authorized to verify products".into());
    }

    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Update product authentication status
    auction.product.is_authenticated = is_authentic;
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "product_verified"), auction_id.clone()),
        (auction_id, is_authentic),
    );

    Ok(())
}

// Add shipping information (seller only)
pub fn add_shipping_info(
    env: Env,
    auction_id: BytesN<32>,
    tracking_number: String,
    carrier: String,
    estimated_delivery: u64,
    shipping_cost: i128,
    recipient_address: String,
) -> Result<(), String> {
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Only the seller can add shipping info
    auction.product.seller.require_auth();

    if auction.status != AuctionStatus::Ended {
        return Err("Can only add shipping info for ended auctions".into());
    }

    if auction.current_highest_bid.is_none() {
        return Err("No winning bid for this auction".into());
    }

    // Create shipping info
    let shipping_info = ShippingInfo {
        status: ShippingStatus::Shipped,
        tracking_number: tracking_number.clone(),
        carrier,
        estimated_delivery,
        shipping_cost,
        recipient_address,
    };

    // Update auction
    auction.shipping = Some(shipping_info);
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "product_shipped"), auction_id.clone()),
        (auction_id, tracking_number),
    );

    Ok(())
}

// Update shipping status
pub fn update_shipping_status(
    env: Env,
    auction_id: BytesN<32>,
    new_status: ShippingStatus,
) -> Result<(), String> {
    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Only the seller can update shipping status
    auction.product.seller.require_auth();

    let mut shipping = auction
        .shipping
        .clone()
        .ok_or("No shipping information available")?;

    // Update status
    shipping.status = new_status;
    auction.shipping = Some(shipping);

    // If delivered, potentially change auction status
    if new_status == ShippingStatus::Delivered {
        auction.status = AuctionStatus::Completed;

        // Emit delivery event
        env.events().publish(
            (Symbol::new(&env, "product_delivered"), auction_id.clone()),
            auction_id.clone(),
        );
    }

    // Update storage
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    Ok(())
}

// Open a dispute (buyer only)
pub fn open_dispute(
    env: Env,
    auction_id: BytesN<32>,
    buyer: Address,
    reason: String,
) -> Result<(), String> {
    buyer.require_auth();

    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    // Check if caller is the highest bidder
    if let Some(highest_bid) = &auction.current_highest_bid {
        if highest_bid.bidder != buyer {
            return Err("Only the highest bidder can open a dispute".into());
        }
    } else {
        return Err("No bids on this auction".into());
    }

    if auction.status != AuctionStatus::Ended && auction.status != AuctionStatus::Completed {
        return Err("Can only open disputes for ended or completed auctions".into());
    }

    if auction.dispute_status != DisputeStatus::None {
        return Err("A dispute is already open for this auction".into());
    }

    // Update auction
    auction.dispute_status = DisputeStatus::Open;
    auction.dispute_reason = Some(reason.clone());
    auction.status = AuctionStatus::Disputed;
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "dispute_opened"), auction_id.clone()),
        (auction_id, reason),
    );

    Ok(())
}

// Resolve a dispute (admin or dispute resolver only)
pub fn resolve_dispute(
    env: Env,
    resolver: Address,
    auction_id: BytesN<32>,
    resolution: DisputeStatus,
) -> Result<(), String> {
    resolver.require_auth();

    // Check if resolver is authorized
    let admin = env
        .storage()
        .instance()
        .get::<_, Address>(&DataKey::Admin)
        .unwrap();
    let resolvers = env
        .storage()
        .instance()
        .get::<_, Vec<Address>>(&DataKey::DisputeResolvers)
        .unwrap();

    if resolver != admin && !resolvers.contains(&resolver) {
        return Err("Not authorized to resolve disputes".into());
    }

    if resolution == DisputeStatus::None || resolution == DisputeStatus::Open {
        return Err("Invalid resolution status".into());
    }

    let mut auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut auction = auctions
        .get(auction_id.clone())
        .ok_or("Auction not found")?;

    if auction.dispute_status != DisputeStatus::Open {
        return Err("No open dispute for this auction".into());
    }

    // Update auction
    auction.dispute_status = resolution;
    auction.status = AuctionStatus::Completed; // Mark as completed after resolution
    auctions.set(auction_id.clone(), auction);
    env.storage().instance().set(&DataKey::Auctions, &auctions);

    // Emit event
    env.events().publish(
        (Symbol::new(&env, "dispute_resolved"), auction_id.clone()),
        (auction_id, resolution),
    );

    Ok(())
}

// Add a product verifier (admin only)
pub fn add_verifier(env: Env, admin: Address, verifier: Address) -> Result<(), String> {
    admin.require_auth();

    // Check if caller is admin
    let stored_admin = env
        .storage()
        .instance()
        .get::<_, Address>(&DataKey::Admin)
        .unwrap();
    if admin != stored_admin {
        return Err("Only admin can add verifiers".into());
    }

    let mut verifiers = env
        .storage()
        .instance()
        .get::<_, Vec<Address>>(&DataKey::ProductVerifiers)
        .unwrap();

    // Check if already a verifier
    if verifiers.contains(&verifier) {
        return Err("Address is already a verifier".into());
    }

    // Add verifier
    verifiers.push_back(verifier);
    env.storage()
        .instance()
        .set(&DataKey::ProductVerifiers, &verifiers);

    Ok(())
}

// Add a dispute resolver (admin only)
pub fn add_resolver(env: Env, admin: Address, resolver: Address) -> Result<(), String> {
    admin.require_auth();

    // Check if caller is admin
    let stored_admin = env
        .storage()
        .instance()
        .get::<_, Address>(&DataKey::Admin)
        .unwrap();
    if admin != stored_admin {
        return Err("Only admin can add dispute resolvers".into());
    }

    let mut resolvers = env
        .storage()
        .instance()
        .get::<_, Vec<Address>>(&DataKey::DisputeResolvers)
        .unwrap();

    // Check if already a resolver
    if resolvers.contains(&resolver) {
        return Err("Address is already a dispute resolver".into());
    }

    // Add resolver
    resolvers.push_back(resolver);
    env.storage()
        .instance()
        .set(&DataKey::DisputeResolvers, &resolvers);

    Ok(())
}

// Get auction details
pub fn get_auction(env: Env, auction_id: BytesN<32>) -> Option<Auction> {
    let auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    auctions.get(auction_id)
}

// Get auctions where user is seller
pub fn get_user_selling_auctions(env: Env, user: Address) -> Vec<BytesN<32>> {
    let key = DataKey::UserSellingAuctions(user);
    env.storage()
        .instance()
        .get::<_, Vec<BytesN<32>>>(&key)
        .unwrap_or(Vec::new(&env))
}

// Get auctions where user has bid
pub fn get_user_bidding_auctions(env: Env, user: Address) -> Vec<BytesN<32>> {
    let key = DataKey::UserBiddingAuctions(user);
    env.storage()
        .instance()
        .get::<_, Vec<BytesN<32>>>(&key)
        .unwrap_or(Vec::new(&env))
}

// Bulk operations: get multiple auctions at once
pub fn get_auctions(env: Env, auction_ids: Vec<BytesN<32>>) -> Vec<Auction> {
    let auctions_map = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let mut result = Vec::new(&env);

    for id in auction_ids.iter() {
        if let Some(auction) = auctions_map.get(id) {
            result.push_back(auction);
        }
    }

    result
}

// Calculate shipping based on location
pub fn calculate_shipping_cost(
    env: Env,
    auction_id: BytesN<32>,
    destination: String,
    shipping_speed: u32,
) -> Result<i128, String> {
    let auctions = env
        .storage()
        .instance()
        .get::<_, Map<BytesN<32>, Auction>>(&DataKey::Auctions)
        .unwrap();
    let auction = auctions.get(auction_id).ok_or("Auction not found")?;

    // This would normally integrate with external shipping APIs
    // For now, use a simplified calculation based on product size and destination

    // Base cost
    let mut cost: i128 = 500; // 5.00 units

    // Add cost based on destination length (simple proxy for distance)
    cost += destination.len() as i128 * 10;

    // Add cost based on shipping speed
    match shipping_speed {
        1 => cost += 1000, // Express
        2 => cost += 500,  // Standard
        _ => cost += 200,  // Economy
    }

    // Inventory count affects shipping (bulk shipping discount)
    if let Some(bid) = &auction.current_highest_bid {
        if bid.quantity > 1 {
            // 10% discount for each additional item
            let discount_factor =
                90_i128.pow(bid.quantity as u32 - 1) / 100_i128.pow(bid.quantity as u32 - 1);
            cost = cost * discount_factor;
        }
    }

    Ok(cost)
}
