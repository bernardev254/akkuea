use soroban_sdk::{contracttype, Address, Map, String, Vec, BytesN};

/// Enhanced Credential structure with tiered verification and W3C compliance
#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub id: BytesN<32>,           // Unique credential identifier
    pub tier: u32,                // Verification tier (1=basic, 2=advanced, 3=premium)
    pub expiration: u64,          // Timestamp for credential expiration
    pub w3c_compliant: bool,      // Indicates W3C verifiable credential compliance
    pub issuer: Address,          // Credential issuer
    pub subject: Address,         // Credential subject
    pub credential_hash: String,  // Hash of credential data
    pub cross_chain_verified: bool, // Cross-chain verification status
    pub renewal_count: u32,       // Number of times renewed
}

/// Enhanced NFT structure with dynamic metadata and templates
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub id: BytesN<32>,           // Unique NFT identifier
    pub metadata: Map<String, String>,  // Dynamic metadata key-value pairs
    pub template_id: u32,         // Visual template for rendering
    pub is_badge: bool,           // Indicates if NFT is an achievement badge
    pub owner: Address,           // Current owner
    pub creation_timestamp: u64,  // When NFT was created
    pub last_update: u64,         // Last metadata update timestamp
    pub upgrade_level: u32,       // Current upgrade level
}

/// Visual template for NFT rendering
#[contracttype]
#[derive(Clone)]
pub struct NFTTemplate {
    pub template_id: u32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub attributes: Map<String, String>,
    pub is_badge_template: bool,
}

/// Achievement badge configuration
#[contracttype]
#[derive(Clone)]
pub struct AchievementBadge {
    pub badge_id: BytesN<32>,
    pub name: String,
    pub description: String,
    pub criteria: String,
    pub template_id: u32,
    pub required_tier: u32,
    pub is_active: bool,
}

/// Educator profile data structure
#[contracttype]
#[derive(Clone)]
pub struct Educator {
    pub address: Address,
    pub name: String,
    pub credentials: Vec<Credential>,
    pub verification_status: bool,
    pub nft_token_id: Option<String>,
    pub verification_timestamp: u64,
    pub specialty_areas: Vec<String>,
    pub verification_level: VerificationLevel,
    pub reviews_count: u32,
    pub ratings: Map<String, (u32, u32)>, // e.g., {"Clarity": (score, weight), "Knowledge": (score, weight)}
    pub owned_nfts: Vec<BytesN<32>>,      // List of owned NFT IDs
    pub achievement_badges: Vec<BytesN<32>>, // List of earned badge IDs
}

/// Enhanced verification levels for educators with tiered system
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum VerificationLevel {
    Pending,
    Basic,      // Tier 1
    Advanced,   // Tier 2
    Expert,     // Tier 3
    Premium,    // Tier 3+ with additional requirements
}

/// Verification request data structure
#[contracttype]
#[derive(Clone)]
pub struct VerificationRequest {
    pub educator: Address,
    pub document_hashes: Vec<String>,
    pub request_timestamp: u64,
    pub status: RequestStatus,
    pub reviewer: Option<Address>,
}

/// Status options for verification requests
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum RequestStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
}

/// Review data structure
#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub review_id: u32,
    pub reviewer: Address,
    pub educator: Address,
    pub ratings: Map<String, u32>,
    pub comment_hash: String,
    pub timestamp: u64,
    pub verifiers: Vec<Address>,
    pub dispute_status: DisputeStatus,
}

/// Dispute data structure
#[contracttype]
#[derive(Clone)]
pub struct Dispute {
    pub review_id: u32,
    pub educator: Address,
    pub reason_hash: String,
    pub status: DisputeStatus,
}

/// Status options for review disputes
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum DisputeStatus {
    None,
    Active,
    Resolved,
}

// A snapshot of analytics data at a specific point in time for trend analysis.
#[contracttype]
#[derive(Clone)]
pub struct AnalyticsSnapshot {
    pub timestamp: u64,
    pub total_verifications: u32,
    pub total_reviews: u32,
    pub total_disputes: u32,
}

// A struct to track the performance metrics of individual reviewers.
#[contracttype]
#[derive(Clone)]
pub struct ReviewerPerformance {
    pub reviews_submitted: u32,
    pub disputes_received: u32, // Number of their reviews that were disputed
}

// The main analytics struct now includes trend and performance data.
#[contracttype]
#[derive(Clone)]
pub struct AnalyticsData {
    pub current_snapshot: AnalyticsSnapshot,
    pub history: Vec<AnalyticsSnapshot>, // For trend analysis
    pub specialty_distribution: Map<String, u32>,
    pub reviewer_performance: Map<Address, ReviewerPerformance>,
}
