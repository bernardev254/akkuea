use soroban_sdk::{Address, String, Vec, contracttype};

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

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum VerificationLevel {
    Pending,
    Basic,
    Advanced,
    Expert,
}

#[contracttype]
#[derive(Clone)]
pub struct VerificationRequest {
    pub educator: Address,
    pub document_hashes: Vec<String>,
    pub request_timestamp: u64,
    pub status: RequestStatus,
    pub reviewer: Option<Address>,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum RequestStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
}

#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub reviewer: Address,
    pub educator: Address,
    pub rating: u32,
    pub timestamp: u64,
} 