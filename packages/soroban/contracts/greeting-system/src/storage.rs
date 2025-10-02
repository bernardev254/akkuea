use soroban_sdk::{contracttype, Address, Env};

use crate::{Error, GreetingReward, PremiumTier};

/// Storage keys for the premium tier system
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    PremiumTier(Address),
    GreetingReward(u64),
    RewardClaimed(u64),
}

/// Save a premium tier to storage
pub fn save_premium_tier(env: &Env, tier: &PremiumTier) -> Result<(), Error> {
    let key = StorageKey::PremiumTier(tier.user.clone());
    env.storage().persistent().set(&key, tier);
    Ok(())
}

/// Load a premium tier from storage
pub fn load_premium_tier(env: &Env, user: &Address) -> Result<PremiumTier, Error> {
    let key = StorageKey::PremiumTier(user.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::TierNotFound)
}

/// Check if a user has a premium tier
pub fn has_premium_tier(env: &Env, user: &Address) -> bool {
    let key = StorageKey::PremiumTier(user.clone());
    env.storage().persistent().has(&key)
}

/// Remove a premium tier from storage
pub fn remove_premium_tier(env: &Env, user: &Address) -> Result<(), Error> {
    let key = StorageKey::PremiumTier(user.clone());
    env.storage().persistent().remove(&key);
    Ok(())
}

/// Save a greeting reward record
pub fn save_greeting_reward(env: &Env, reward: &GreetingReward) -> Result<(), Error> {
    let key = StorageKey::GreetingReward(reward.greeting_id);
    env.storage().persistent().set(&key, reward);
    Ok(())
}

/// Load a greeting reward record
pub fn load_greeting_reward(env: &Env, greeting_id: &u64) -> Option<GreetingReward> {
    let key = StorageKey::GreetingReward(*greeting_id);
    env.storage().persistent().get(&key)
}

/// Mark a greeting as having a reward claimed
pub fn mark_reward_claimed(env: &Env, greeting_id: &u64) {
    let key = StorageKey::RewardClaimed(*greeting_id);
    env.storage().persistent().set(&key, &true);
}

/// Check if a reward has been claimed for a greeting
pub fn is_reward_claimed(env: &Env, greeting_id: &u64) -> bool {
    let key = StorageKey::RewardClaimed(*greeting_id);
    env.storage().persistent().get(&key).unwrap_or(false)
}
