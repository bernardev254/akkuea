#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

mod publish;
mod vote;
mod verify;
mod storage;

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
}

#[cfg(test)]
mod tests;