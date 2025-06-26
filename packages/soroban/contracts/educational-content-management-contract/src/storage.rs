use soroban_sdk::{contracttype, Address, BytesN, Env, String, Vec};

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
    pub is_verified: bool,
}

// Keys for contract data
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Content(u64),            // Content ID -> Content
    ContentCounter,          // Counter for content IDs
    UserVotes(Address, u64), // (User, Content ID) -> Has voted?
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
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or_else(|| panic!("content with ID {} not found", content_id))
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
