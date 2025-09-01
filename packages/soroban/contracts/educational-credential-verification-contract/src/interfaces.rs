use soroban_sdk::{Address, Env, Map, String, Vec, BytesN};
use crate::datatype::{AnalyticsData, Educator, VerificationLevel, Review, Dispute, ReviewerPerformance, Credential, NFT, NFTTemplate, AchievementBadge};

pub trait EducatorVerificationInterface {
    // --- Administration Functions ---
    fn initialize(env: Env, admin: Address);
    fn add_reviewer(env: Env, admin: Address, reviewer: Address);
    fn remove_reviewer(env: Env, admin: Address, reviewer: Address);
    fn add_authorized_institution(env: Env, admin: Address, institution_id: String);
    
    // --- Educator Functions ---
    fn register_educator(env: Env, educator_address: Address, name: String, credential_hashes: Vec<String>, specialty_areas: Vec<String>) -> Address;
    fn update_educator_profile(env: Env, educator_address: Address, name: Option<String>, specialty_areas: Option<Vec<String>>) -> bool;
    fn add_credentials(env: Env, educator_address: Address, new_credentials: Vec<String>) -> bool;
    
    // --- Verification Functions ---
    fn verify_educator(env: Env, reviewer: Address, educator_address: Address, verification_level: VerificationLevel);
    fn revoke_verification(env: Env, admin: Address, educator_address: Address, reason: String);
    fn add_verified_credential(env: Env, reviewer: Address, credential: String);

    // --- Review System Functions ---
    fn submit_review(env: Env, reviewer: Address, educator_address: Address, ratings: Map<String, u32>, comment_hash: String);
    fn verify_review(env: Env, verifier: Address, educator_address: Address, review_id: u32);
    fn dispute_review(env: Env, educator: Address, review_id: u32, reason_hash: String);
    fn resolve_dispute(env: Env, admin: Address, educator_address: Address, review_id: u32);

    // --- Query Functions ---
    fn get_educator(env: Env, educator_address: Address) -> Option<Educator>;
    fn get_verified_educators(env: Env) -> Vec<Address>;
    fn get_educators_by_specialty(env: Env, specialty: String) -> Vec<Address>;
    fn get_educator_reviews(env: Env, educator_address: Address) -> Vec<Review>;

    // --- Enhanced Credential Functions ---
    fn create_credential(env: Env, issuer: Address, subject: Address, credential_hash: String, tier: u32, w3c_compliant: bool) -> BytesN<32>;
    fn renew_credential(env: Env, issuer: Address, credential_id: BytesN<32>) -> bool;
    fn verify_cross_chain(env: Env, verifier: Address, credential_id: BytesN<32>, chain_id: u32, verification_hash: String) -> bool;
    fn get_credential_info(env: Env, credential_id: BytesN<32>) -> Option<Credential>;
    fn get_credentials_by_subject(env: Env, subject: Address) -> Vec<Credential>;

    // --- Dynamic NFT Functions ---
    fn create_dynamic_nft(env: Env, admin: Address, owner: Address, template_id: u32, is_badge: bool, initial_metadata: Map<String, String>) -> BytesN<32>;
    fn update_nft_metadata(env: Env, owner: Address, nft_id: BytesN<32>, new_metadata: Map<String, String>) -> bool;
    fn upgrade_nft(env: Env, owner: Address, nft_id: BytesN<32>, additional_metadata: Map<String, String>) -> bool;
    fn list_nfts(env: Env, owner: Address) -> Vec<NFT>;
    fn get_nft_info(env: Env, nft_id: BytesN<32>) -> Option<NFT>;

    // --- NFT Template Functions ---
    fn create_nft_template(env: Env, admin: Address, name: String, description: String, image_url: String, attributes: Map<String, String>, is_badge_template: bool) -> u32;
    fn get_nft_template(env: Env, template_id: u32) -> Option<NFTTemplate>;

    // --- Achievement Badge Functions ---
    fn issue_badge(env: Env, admin: Address, educator: Address, badge_name: String, badge_description: String, criteria: String, required_tier: u32, template_id: u32) -> BytesN<32>;
    fn get_achievement_badge(env: Env, badge_id: BytesN<32>) -> Option<AchievementBadge>;
    fn get_disputes(env: Env) -> Vec<Dispute>;

    // --- Analytics Functions (UPDATED) ---
    fn get_analytics(env: Env) -> AnalyticsData;
    fn recalculate_analytics(env: Env, admin: Address);
    // Function to get performance metrics for a specific reviewer.
    fn get_reviewer_performance(env: Env, reviewer: Address) -> Option<ReviewerPerformance>;
}
