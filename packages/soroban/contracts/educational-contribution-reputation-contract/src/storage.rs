use crate::error::Error;
use crate::types::*;
use soroban_sdk::{Env, Map, String, Vec};

/// Storage operations for the reputation contract

/// Get next available user ID
pub fn get_next_user_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextUserId)
        .unwrap_or(1u64)
}

/// Increment and return next user ID
pub fn increment_user_id(env: &Env) -> u64 {
    let next_id = get_next_user_id(env);
    env.storage()
        .instance()
        .set(&DataKey::NextUserId, &(next_id + 1));
    next_id
}

/// Get next available token ID
pub fn get_next_token_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextTokenId)
        .unwrap_or(1u64)
}

/// Increment and return next token ID
pub fn increment_token_id(env: &Env) -> u64 {
    let next_id = get_next_token_id(env);
    env.storage()
        .instance()
        .set(&DataKey::NextTokenId, &(next_id + 1));
    next_id
}

/// Get next available dispute ID
pub fn get_next_dispute_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextDisputeId)
        .unwrap_or(1u64)
}

/// Increment and return next dispute ID
pub fn increment_dispute_id(env: &Env) -> u64 {
    let next_id = get_next_dispute_id(env);
    env.storage()
        .instance()
        .set(&DataKey::NextDisputeId, &(next_id + 1));
    next_id
}

/// Store user data
pub fn store_user(env: &Env, user: &User) {
    env.storage().instance().set(&DataKey::User(user.id), user);
}

/// Get user data
pub fn get_user(env: &Env, user_id: u64) -> Result<User, Error> {
    env.storage()
        .instance()
        .get(&DataKey::User(user_id))
        .ok_or(Error::UserNotFound)
}

/// Store reputation data
pub fn store_reputation(env: &Env, reputation: &Reputation) {
    let key = DataKey::Reputation(reputation.user_id, reputation.subject.clone());
    env.storage().instance().set(&key, reputation);
}

/// Get reputation data
pub fn get_reputation(env: &Env, user_id: u64, subject: String) -> Result<Reputation, Error> {
    let key = DataKey::Reputation(user_id, subject);
    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ReputationNotFound)
}

/// Store credential token
pub fn store_credential(env: &Env, credential: &CredentialToken) {
    env.storage()
        .instance()
        .set(&DataKey::Credential(credential.token_id), credential);
}

/// Get credential token
pub fn get_credential(env: &Env, token_id: u64) -> Result<CredentialToken, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Credential(token_id))
        .ok_or(Error::TokenNotFound)
}

/// Store dispute data
pub fn store_dispute(env: &Env, dispute: &Dispute) {
    env.storage()
        .instance()
        .set(&DataKey::Dispute(dispute.id), dispute);
}

/// Get dispute data
pub fn get_dispute(env: &Env, dispute_id: u64) -> Result<Dispute, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Dispute(dispute_id))
        .ok_or(Error::DisputeNotFound)
}

/// Store user disputes list
pub fn store_user_disputes(env: &Env, user_id: u64, disputes: &Vec<u64>) {
    env.storage()
        .instance()
        .set(&DataKey::UserDisputes(user_id), disputes);
}

/// Get user disputes list
pub fn get_user_disputes(env: &Env, user_id: u64) -> Vec<u64> {
    env.storage()
        .instance()
        .get(&DataKey::UserDisputes(user_id))
        .unwrap_or(Vec::new(env))
}

/// Store recovery plan
pub fn store_recovery_plan(env: &Env, plan: &RecoveryPlan) {
    env.storage()
        .instance()
        .set(&DataKey::RecoveryPlan(plan.user_id), plan);
}

/// Get recovery plan
pub fn get_recovery_plan(env: &Env, user_id: u64) -> Result<RecoveryPlan, Error> {
    env.storage()
        .instance()
        .get(&DataKey::RecoveryPlan(user_id))
        .ok_or(Error::RecoveryNotAllowed)
}

/// Store probation status
pub fn store_probation_status(env: &Env, status: &ProbationStatus) {
    env.storage()
        .instance()
        .set(&DataKey::ProbationStatus(status.user_id), status);
}

/// Get probation status
pub fn get_probation_status(env: &Env, user_id: u64) -> ProbationStatus {
    env.storage()
        .instance()
        .get(&DataKey::ProbationStatus(user_id))
        .unwrap_or(ProbationStatus {
            user_id,
            active: false,
            start_date: 0,
            end_date: 0,
            reason: String::from_str(env, ""),
            restrictions: Map::new(env),
        })
}

/// Store reputation history
pub fn store_reputation_history(env: &Env, history: &ReputationHistory) {
    let key = DataKey::ReputationHistory(history.user_id, history.subject.clone());
    env.storage().instance().set(&key, history);
}

/// Get reputation history
pub fn get_reputation_history(env: &Env, user_id: u64, subject: String) -> ReputationHistory {
    let key = DataKey::ReputationHistory(user_id, subject.clone());
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(ReputationHistory {
            user_id,
            subject,
            scores: Map::new(env),
            changes: Map::new(env),
        })
}

/// Store analytics data
pub fn store_analytics(env: &Env, analytics: &Analytics) {
    env.storage()
        .instance()
        .set(&DataKey::Analytics(analytics.key.clone()), analytics);
}

/// Get analytics data
pub fn get_analytics(env: &Env, key: String) -> Option<Analytics> {
    env.storage().instance().get(&DataKey::Analytics(key))
}

/// Check if user exists
pub fn user_exists(env: &Env, user_id: u64) -> bool {
    env.storage().instance().has(&DataKey::User(user_id))
}

/// Check if reputation exists
pub fn reputation_exists(env: &Env, user_id: u64, subject: String) -> bool {
    env.storage()
        .instance()
        .has(&DataKey::Reputation(user_id, subject))
}

/// Check if dispute exists
pub fn dispute_exists(env: &Env, dispute_id: u64) -> bool {
    env.storage().instance().has(&DataKey::Dispute(dispute_id))
}

/// Get all users (for analytics purposes)
pub fn get_all_user_ids(env: &Env) -> Vec<u64> {
    let mut user_ids = Vec::new(env);
    let max_user_id = get_next_user_id(env);

    for user_id in 1..max_user_id {
        if user_exists(env, user_id) {
            user_ids.push_back(user_id);
        }
    }

    user_ids
}

/// Get all disputes (for analytics purposes)
pub fn get_all_dispute_ids(env: &Env) -> Vec<u64> {
    let mut dispute_ids = Vec::new(env);
    let max_dispute_id = get_next_dispute_id(env);

    for dispute_id in 1..max_dispute_id {
        if dispute_exists(env, dispute_id) {
            dispute_ids.push_back(dispute_id);
        }
    }

    dispute_ids
}

/// Clean up expired probations
pub fn cleanup_expired_probations(env: &Env) {
    let current_time = env.ledger().timestamp();
    let user_ids = get_all_user_ids(env);

    for user_id in user_ids.iter() {
        let mut probation = get_probation_status(env, user_id);
        if probation.active && current_time > probation.end_date {
            probation.active = false;
            store_probation_status(env, &probation);
        }
    }
}
