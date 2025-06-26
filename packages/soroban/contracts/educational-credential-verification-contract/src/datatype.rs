use soroban_sdk::{contracttype, Address, String, Vec};

/// Educator profile data structure
/// Contains all information about an educator including their credentials and verification status
#[contracttype]
#[derive(Clone)]
pub struct Educator {
    pub address: Address,
    pub name: String,
    pub credentials: Vec<String>,
    pub verification_status: bool,
    pub nft_token_id: Option<String>,
    pub verification_timestamp: u64,
    pub specialty_areas: Vec<String>,
    pub verification_level: VerificationLevel,
    pub reviews_count: u32,
    pub rating: u32,
}

/// Verification levels for educators
/// Represents the different tiers of verification an educator can achieve
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum VerificationLevel {
    Pending,
    Basic,
    Advanced,
    Expert,
}

/// Verification request data structure
/// Used to track the status of verification requests from educators
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
/// Contains information about reviews submitted for educators
#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub reviewer: Address,
    pub educator: Address,
    pub rating: u32,
    pub timestamp: u64,
}
