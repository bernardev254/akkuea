use soroban_sdk::{contracttype, Address, BytesN, Env, String, Vec};

// Define the verification tiers
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VerificationLevel {
    None = 0,
    Peer = 1,
    Expert = 2,
    Institutional = 3,
}

// Content data structure as per requirements
#[contracttype]
pub struct Content {
    pub id: u64,
    pub creator: Address,
    pub title: String,
    pub content_hash: BytesN<32>,
    pub creation_date: u64,
    pub subject_tags: Vec<String>,
    pub upvotes: u32,
    pub verification_level: VerificationLevel,
}

// Keys for contract data
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Content(u64),
    ContentCounter,
    UserVotes(Address, u64),
}

// --- Advanced Verification and Moderation Additions ---

#[contracttype]
pub struct VerificationRecord {
    pub verifier: Address,
    pub level: VerificationLevel,
    pub timestamp: u64,
    pub expiration: Option<u64>,
    pub delegated_by: Option<Address>,
    pub reputation_snapshot: Option<u32>,
}

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub struct Delegation {
    pub delegator: Address,
    pub delegatee: Address,
    pub since: u64,
    pub until: Option<u64>,
}

#[contracttype]
pub struct Flag {
    pub content_id: u64,
    pub flagger: Address,
    pub reason: String,
    pub timestamp: u64,
}

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ModerationStatus {
    Pending,
    Approved,
    Rejected,
    Removed,
    UnderDispute,
}

#[contracttype]
pub struct ModerationAction {
    pub content_id: u64,
    pub moderator: Address,
    pub action: ModerationStatus,
    pub reason: String,
    pub timestamp: u64,
}

#[contracttype]
pub struct Dispute {
    pub dispute_id: u64,
    pub content_id: u64,
    pub creator: Address,
    pub reason: String,
    pub status: ModerationStatus,
    pub created_at: u64,
    pub resolved_at: Option<u64>,
    pub resolver: Option<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum AdvDataKey {
    VerificationRecord(u64), // content_id -> Vec<VerificationRecord>
    Delegation(Address),     // delegator -> Vec<Delegation>
    Flag(u64),              // content_id -> Vec<Flag>
    Moderation(u64),        // content_id -> Vec<ModerationAction>
    Dispute(u64),           // dispute_id -> Dispute
    DisputeCounter,         // u64
}

// Get the next content ID and increment the counter
pub fn get_next_content_id(env: &Env) -> u64 {
    let key = DataKey::ContentCounter;
    let id = env.storage().instance().get(&key).unwrap_or(0u64);
    env.storage().instance().set(&key, &(id + 1));
    id
}

// Store content in contract storage
pub fn save_content(env: &Env, content: &Content) {
    let key = DataKey::Content(content.id);
    env.storage().instance().set(&key, content);
}

// Retrieve content from contract storage
pub fn get_content(env: &Env, content_id: u64) -> Content {
    let key = DataKey::Content(content_id);
    env.storage().instance().get(&key).unwrap_or_else(|| {
        panic!("content with ID {} not found", content_id)
    })
}

// Record a user's vote for a specific content
pub fn record_user_vote(env: &Env, voter: Address, content_id: u64) {
    let key = DataKey::UserVotes(voter, content_id);
    env.storage().instance().set(&key, &true);
}

// Check if a user has already voted for a specific content
pub fn has_user_voted(env: &Env, voter: &Address, content_id: u64) -> bool {
    let key = DataKey::UserVotes(voter.clone(), content_id);
    env.storage().instance().has(&key)
}

// Get the current content counter (total number of content items created)
pub fn get_content_counter(env: &Env) -> u64 {
    let key = DataKey::ContentCounter;
    env.storage().instance().get(&key).unwrap_or(0u64)
}

// Check if content exists by ID
pub fn content_exists(env: &Env, content_id: u64) -> bool {
    let key = DataKey::Content(content_id);
    env.storage().instance().has(&key)
}

// Get all existing content IDs (for filtering operations)
pub fn get_all_content_ids(env: &Env) -> Vec<u64> {
    let mut content_ids = Vec::new(env);
    let total_content = get_content_counter(env);

    for id in 0..total_content {
        if content_exists(env, id) {
            content_ids.push_back(id);
        }
    }

    content_ids
}