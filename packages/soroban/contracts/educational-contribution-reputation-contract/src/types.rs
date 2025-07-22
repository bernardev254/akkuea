use soroban_sdk::{contracttype, Map, String};

#[contracttype]
pub enum DataKey {
    User(u64),                      // User ID -> User data
    Credential(u64),                // Token ID -> CredentialToken data
    Reputation(u64, String),        // (User ID, Subject) -> Reputation data
    NextUserId,                     // Counter for user IDs
    NextTokenId,                    // Counter for token IDs
    Dispute(u64),                   // Dispute ID -> Dispute data
    NextDisputeId,                  // Counter for dispute IDs
    UserDisputes(u64),              // User ID -> List of dispute IDs
    RecoveryPlan(u64),              // User ID -> Recovery plan
    ProbationStatus(u64),           // User ID -> Probation status
    ReputationHistory(u64, String), // (User ID, Subject) -> Historical reputation data
    Analytics(String),              // Analytics key -> Analytics data
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

#[contracttype]
#[derive(Clone)]
pub struct Dispute {
    pub id: u64,
    pub user_id: u64,
    pub subject: String,
    pub original_score: u32,
    pub disputed_score: u32,
    pub evidence: String,
    pub status: DisputeStatus,
    pub created_at: u64,
    pub resolved_at: Option<u64>,
    pub resolver: Option<String>,
}

#[contracttype]
#[derive(Clone)]
pub enum DisputeStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
}

#[contracttype]
#[derive(Clone)]
pub struct RecoveryPlan {
    pub user_id: u64,
    pub target_score: u32,
    pub milestones: Map<String, u32>, // Subject -> Target score
    pub created_at: u64,
    pub deadline: u64,
    pub progress: Map<String, u32>, // Subject -> Current progress
    pub completed: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct ProbationStatus {
    pub user_id: u64,
    pub active: bool,
    pub start_date: u64,
    pub end_date: u64,
    pub reason: String,
    pub restrictions: Map<String, bool>, // Restriction type -> Active
}

#[contracttype]
#[derive(Clone)]
pub struct ReputationHistory {
    pub user_id: u64,
    pub subject: String,
    pub scores: Map<u64, u32>,     // Timestamp -> Score
    pub changes: Map<u64, String>, // Timestamp -> Change reason
}

#[contracttype]
#[derive(Clone)]
pub struct Analytics {
    pub key: String,
    pub data: Map<String, u32>, // Metric name -> Value
    pub trends: Map<u64, u32>,  // Timestamp -> Trend value
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct DomainExpertise {
    pub domain: String,
    pub experts: Map<u64, u32>, // User ID -> Expertise score
    pub average_score: u32,
    pub total_contributors: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct PeerBenchmark {
    pub user_id: u64,
    pub subject: String,
    pub user_score: u32,
    pub peer_average: u32,
    pub percentile: u32,
    pub rank: u32,
    pub total_peers: u32,
}
