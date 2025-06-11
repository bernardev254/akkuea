#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

mod credentials;
mod error;
mod expertise;
mod reputation;
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
}
