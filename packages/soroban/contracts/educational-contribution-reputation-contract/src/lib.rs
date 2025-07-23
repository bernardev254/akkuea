#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};

mod analytics;
mod credentials;
mod error;
mod expertise;
mod recovery;
mod reputation;
mod storage;
mod test;
mod types;
mod verify;

pub use error::Error;
pub use types::*;

#[contract]
pub struct ContributorReputation;

#[contractimpl]
impl ContributorReputation {
    // Initialize a new user
    pub fn initialize_user(env: Env, caller: Address, name: String) -> Result<u64, Error> {
        caller.require_auth();

        let user_id = env
            .storage()
            .instance()
            .get(&DataKey::NextUserId)
            .unwrap_or(1u64);
        env.storage()
            .instance()
            .set(&DataKey::NextUserId, &(user_id + 1));

        let user = User {
            id: user_id,
            name,
            expertise_areas: Map::new(&env),
            verified: false,
        };
        env.storage().instance().set(&DataKey::User(user_id), &user);
        Ok(user_id)
    }

    pub fn get_user(env: Env, user_id: u64) -> Result<User, Error> {
        env.storage()
            .instance()
            .get(&DataKey::User(user_id))
            .ok_or(Error::UserNotFound)
    }

    // Reputation functions
    pub fn update_reputation(
        env: Env,
        caller: Address,
        user_id: u64,
        subject: String,
        score: u32,
    ) -> Result<(), Error> {
        reputation::update_reputation(env, caller, user_id, subject, score)
    }

    pub fn get_reputation(env: Env, user_id: u64, subject: String) -> Result<u32, Error> {
        reputation::get_reputation(env, user_id, subject)
    }

    // Credential functions
    pub fn mint_credential_token(env: Env, caller: Address, user_id: u64) -> Result<u64, Error> {
        credentials::mint_credential_token(env, caller, user_id)
    }

    // Expertise functions
    pub fn update_expertise_areas(
        env: Env,
        caller: Address,
        user_id: u64,
        expertise_areas: Map<String, u32>,
    ) -> Result<(), Error> {
        expertise::update_expertise_areas(env, caller, user_id, expertise_areas)
    }

    pub fn get_expertise_areas(env: Env, user_id: u64) -> Result<Map<String, u32>, Error> {
        expertise::get_expertise_areas(env, user_id)
    }

    // Verification functions
    pub fn verify_user(
        env: Env,
        caller: Address,
        user_id: u64,
        verification_details: String,
    ) -> Result<(), Error> {
        verify::verify_user(env, caller, user_id, verification_details)
    }

    pub fn verify_content(
        env: Env,
        caller: Address,
        content_id: u64,
        subject: String,
    ) -> Result<(), Error> {
        verify::verify_content(env, caller, content_id, subject)
    }

    // Recovery functions
    pub fn submit_dispute(
        env: Env,
        caller: Address,
        user_id: u64,
        subject: String,
        disputed_score: u32,
        evidence: String,
    ) -> Result<u64, Error> {
        recovery::submit_dispute(env, caller, user_id, subject, disputed_score, evidence)
    }

    pub fn resolve_dispute(
        env: Env,
        caller: Address,
        dispute_id: u64,
        approved: bool,
        resolver_name: String,
    ) -> Result<(), Error> {
        recovery::resolve_dispute(env, caller, dispute_id, approved, resolver_name)
    }

    pub fn create_recovery_plan(
        env: Env,
        caller: Address,
        user_id: u64,
        target_score: u32,
        milestones: Map<String, u32>,
        duration_days: u32,
    ) -> Result<(), Error> {
        recovery::create_recovery_plan(
            env,
            caller,
            user_id,
            target_score,
            milestones,
            duration_days,
        )
    }

    pub fn update_recovery_progress(
        env: Env,
        caller: Address,
        user_id: u64,
        subject: String,
        new_score: u32,
    ) -> Result<(), Error> {
        recovery::update_recovery_progress(env, caller, user_id, subject, new_score)
    }

    pub fn set_probation(
        env: Env,
        caller: Address,
        user_id: u64,
        duration_days: u32,
        reason: String,
        restrictions: Map<String, bool>,
    ) -> Result<(), Error> {
        recovery::set_probation(env, caller, user_id, duration_days, reason, restrictions)
    }

    pub fn is_on_probation(env: Env, user_id: u64) -> Result<bool, Error> {
        recovery::is_on_probation(env, user_id)
    }

    pub fn get_recovery_plan(env: Env, user_id: u64) -> Result<RecoveryPlan, Error> {
        recovery::get_recovery_plan(env, user_id)
    }

    pub fn get_dispute(env: Env, dispute_id: u64) -> Result<Dispute, Error> {
        recovery::get_dispute(env, dispute_id)
    }

    pub fn get_user_disputes(env: Env, user_id: u64) -> Result<Vec<u64>, Error> {
        recovery::get_user_disputes(env, user_id)
    }

    // Analytics functions
    pub fn generate_user_analytics(
        env: Env,
        user_id: u64,
        time_range_days: u32,
    ) -> Result<Analytics, Error> {
        analytics::generate_user_analytics(env, user_id, time_range_days)
    }

    pub fn generate_domain_expertise(env: Env, domain: String) -> Result<DomainExpertise, Error> {
        analytics::generate_domain_expertise(env, domain)
    }

    pub fn generate_peer_benchmark(
        env: Env,
        user_id: u64,
        subject: String,
    ) -> Result<PeerBenchmark, Error> {
        analytics::generate_peer_benchmark(env, user_id, subject)
    }

    pub fn predict_reputation_development(
        env: Env,
        user_id: u64,
        subject: String,
        prediction_days: u32,
    ) -> Result<u32, Error> {
        analytics::predict_reputation_development(env, user_id, subject, prediction_days)
    }

    pub fn get_reputation_trends(
        env: Env,
        user_id: u64,
        subject: String,
        days: u32,
    ) -> Result<Map<u64, u32>, Error> {
        analytics::get_reputation_trends(env, user_id, subject, days)
    }

    pub fn calculate_platform_analytics(env: Env) -> Result<Analytics, Error> {
        analytics::calculate_platform_analytics(env)
    }
}
