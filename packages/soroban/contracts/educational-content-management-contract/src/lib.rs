#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

mod publish;
mod vote;
mod verify;
mod storage;
mod moderation;

pub use crate::storage::{Content, VerificationLevel};

#[contract]
pub struct TokenizedEducationalContent;

#[contractimpl]
impl TokenizedEducationalContent {
    pub fn publish_content(
        env: Env,
        creator: Address,
        title: String,
        content_hash: BytesN<32>,
        subject_tags: Vec<String>,
    ) -> u64 {
        creator.require_auth();
        publish::publish_content(&env, creator, title, content_hash, subject_tags)
    }

    pub fn upvote_content(env: Env, content_id: u64, voter: Address) -> u32 {
        voter.require_auth();
        vote::upvote_content(&env, content_id, voter)
    }

    pub fn verify_content(
        env: Env,
        content_id: u64,
        verifier: Address,
        level: VerificationLevel,
    ) -> VerificationLevel {
        verifier.require_auth();
        verify::verify_content(&env, content_id, verifier, level)
    }

    pub fn get_content(env: Env, content_id: u64) -> Content {
        storage::get_content(&env, content_id)
    }

    /// Filter and retrieve only verified content
    /// Returns a vector of all content items where verification_level > None
    /// This is a view-only function that does not modify contract state
    pub fn filter_by_verification(env: Env) -> Vec<Content> {
        let mut verified_content = Vec::new(&env);
        let all_content_ids = storage::get_all_content_ids(&env);

        for i in 0..all_content_ids.len() {
            let content_id = all_content_ids.get(i).unwrap();
            let content = storage::get_content(&env, content_id);

            if content.verification_level > VerificationLevel::None {
                verified_content.push_back(content);
            }
        }

        verified_content
    }


     pub fn filter_by_verification_level(env: Env, level: VerificationLevel) -> Vec<Content> {
        let mut filtered_content = Vec::new(&env);
        let all_content_ids = storage::get_all_content_ids(&env);

        for i in 0..all_content_ids.len() {
            let content_id = all_content_ids.get(i).unwrap();
            let content = storage::get_content(&env, content_id);

            if content.verification_level == level {
                filtered_content.push_back(content);
            }
        }

        filtered_content
    }

    /// Filter and retrieve content with upvotes greater than or equal to the minimum threshold
    /// Returns a vector of all content items where upvotes >= min_upvotes
    /// This is a view-only function that does not modify contract state
    pub fn filter_by_min_upvotes(env: Env, min_upvotes: u32) -> Vec<Content> {
        let mut popular_content = Vec::new(&env);
        let all_content_ids = storage::get_all_content_ids(&env);

        for i in 0..all_content_ids.len() {
            let content_id = all_content_ids.get(i).unwrap();
            let content = storage::get_content(&env, content_id);

            if content.upvotes >= min_upvotes {
                popular_content.push_back(content);
            }
        }

        popular_content
    }

    // --- Advanced Verification ---
    pub fn verify_content_advanced(
        env: Env,
        content_id: u64,
        verifier: Address,
        level: VerificationLevel,
        delegated_by: Option<Address>,
        min_reputation: u32,
        expiration_secs: Option<u64>,
    ) -> VerificationLevel {
        verify::verify_content_advanced(
            &env,
            content_id,
            verifier,
            level,
            delegated_by,
            min_reputation,
            expiration_secs,
        )
    }

    pub fn renew_verification(
        env: Env,
        content_id: u64,
        verifier: Address,
        new_expiration_secs: u64,
    ) {
        verify::renew_verification(&env, content_id, verifier, new_expiration_secs)
    }

    pub fn delegate_verification(
        env: Env,
        delegator: Address,
        delegatee: Address,
        until: Option<u64>,
    ) {
        verify::delegate_verification(&env, delegator, delegatee, until)
    }

    pub fn revoke_delegation(
        env: Env,
        delegator: Address,
        delegatee: Address,
    ) {
        verify::revoke_delegation(&env, delegator, delegatee)
    }

    // --- Moderation ---
    pub fn flag_content(
        env: Env,
        content_id: u64,
        flagger: Address,
        reason: String,
    ) {
        moderation::flag_content(&env, content_id, flagger, reason)
    }

    pub fn get_flags(env: Env, content_id: u64) -> Vec<crate::storage::Flag> {
        moderation::get_flags(&env, content_id)
    }

    pub fn moderate_content(
        env: Env,
        content_id: u64,
        moderator: Address,
        action: crate::storage::ModerationStatus,
        reason: String,
    ) {
        moderation::moderate_content(&env, content_id, moderator, action, reason)
    }

    pub fn get_moderation_history(env: Env, content_id: u64) -> Vec<crate::storage::ModerationAction> {
        moderation::get_moderation_history(&env, content_id)
    }

    pub fn create_dispute(
        env: Env,
        content_id: u64,
        creator: Address,
        reason: String,
    ) -> u64 {
        moderation::create_dispute(&env, content_id, creator, reason)
    }

    pub fn resolve_dispute(
        env: Env,
        dispute_id: u64,
        resolver: Address,
        approve: bool,
    ) {
        moderation::resolve_dispute(&env, dispute_id, resolver, approve)
    }

    pub fn get_dispute(env: Env, dispute_id: u64) -> Option<crate::storage::Dispute> {
        moderation::get_dispute(&env, dispute_id)
    }
}

#[cfg(test)]
mod tests;