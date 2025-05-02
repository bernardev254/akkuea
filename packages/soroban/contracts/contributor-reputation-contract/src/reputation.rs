use crate::error::Error;
use crate::types::*;
use soroban_sdk::{Address, Env, String};

pub fn update_reputation(
    env: Env,
    caller: Address,
    user_id: u64,
    subject: String,
    score: u32,
) -> Result<(), Error> {
    caller.require_auth();

    // Only verified users can update reputation
    let user: User = env
        .storage()
        .instance()
        .get(&DataKey::User(user_id))
        .ok_or(Error::UserNotFound)?;
    if !user.verified {
        return Err(Error::NotVerified);
    }

    // Update or create reputation entry
    let reputation_key = DataKey::Reputation(user_id, subject.clone());
    let reputation = Reputation {
        user_id,
        subject,
        score,
    };
    env.storage().instance().set(&reputation_key, &reputation);
    Ok(())
}

pub fn get_reputation(env: Env, user_id: u64, subject: String) -> Result<u32, Error> {
    let reputation_key = DataKey::Reputation(user_id, subject);
    let reputation: Reputation = env
        .storage()
        .instance()
        .get(&reputation_key)
        .ok_or(Error::ReputationNotFound)?;
    Ok(reputation.score)
}
