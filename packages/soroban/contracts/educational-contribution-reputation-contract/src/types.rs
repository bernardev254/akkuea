use soroban_sdk::{contracttype, Map, String};

#[contracttype]
pub enum DataKey {
    User(u64),               // User ID -> User data
    Credential(u64),         // Token ID -> CredentialToken data
    Reputation(u64, String), // (User ID, Subject) -> Reputation data
    NextUserId,              // Counter for user IDs
    NextTokenId,             // Counter for token IDs
}

#[contracttype]
#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub expertise_areas: Map<String, u32>, // Subject -> Expertise level
    pub verified: bool,                    // Verification status
}

#[contracttype]
#[derive(Clone)]
pub struct CredentialToken {
    pub user_id: u64,
    pub token_id: u64,  // Non-transferable token ID
    pub issued_at: u64, // Timestamp of issuance
}

#[contracttype]
#[derive(Clone)]
pub struct Reputation {
    pub user_id: u64,
    pub subject: String, // Subject area
    pub score: u32,      // Reputation score
}
