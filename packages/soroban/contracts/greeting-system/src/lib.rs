#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod datatype;
mod error;
mod events;
mod interface;
mod storage;
mod utils;

pub use datatype::*;
pub use error::*;
pub use events::*;
pub use interface::*;
pub use storage::*;
pub use utils::*;

#[contract]
pub struct GreetingSystem;

#[contractimpl]
impl GreetingSystem {
    /// Assign a premium tier to a user based on their contribution
    pub fn assign_premium_tier(
        env: Env,
        user: Address,
        contribution: i128,
    ) -> Result<(), Error> {
        verify_user_authorization(&env, &user)?;
        validate_contribution(contribution)?;

        if has_premium_tier(&env, &user) {
            return Err(Error::TierAlreadyExists);
        }

        let tier_level = TierLevel::from_contribution(contribution);
        let features = tier_level.get_features();
        let timestamp = get_current_timestamp(&env);

        let premium_tier = PremiumTier {
            user: user.clone(),
            tier: tier_level.clone(),
            contribution,
            assigned_at: timestamp,
            features,
        };

        save_premium_tier(&env, &premium_tier)?;

        let event = TierAssignmentEvent {
            user: user.clone(),
            tier: tier_level,
            contribution,
            timestamp,
        };
        emit_tier_assigned(&env, &event)?;

        Ok(())
    }

    /// Upgrade a user's premium tier with additional contribution
    pub fn upgrade_premium_tier(
        env: Env,
        user: Address,
        additional_contribution: i128,
    ) -> Result<(), Error> {
        verify_user_authorization(&env, &user)?;
        validate_contribution(additional_contribution)?;

        let mut existing_tier = load_premium_tier(&env, &user)?;
        let new_total_contribution = existing_tier.contribution + additional_contribution;
        let new_tier_level = TierLevel::from_contribution(new_total_contribution);

        if new_tier_level < existing_tier.tier {
            return Err(Error::DowngradeNotAllowed);
        }

        let old_tier = existing_tier.tier.clone();
        existing_tier.tier = new_tier_level.clone();
        existing_tier.contribution = new_total_contribution;
        existing_tier.features = new_tier_level.get_features();

        save_premium_tier(&env, &existing_tier)?;

        let timestamp = get_current_timestamp(&env);
        let event = TierUpgradeEvent {
            user: user.clone(),
            old_tier,
            new_tier: new_tier_level,
            contribution: new_total_contribution,
            timestamp,
        };
        emit_tier_upgraded(&env, &event)?;

        Ok(())
    }

    /// Get the premium status of a user
    pub fn get_premium_status(env: Env, user: Address) -> Result<PremiumTier, Error> {
        load_premium_tier(&env, &user)
    }

    /// Get features available to a user
    pub fn get_user_features(env: Env, user: Address) -> Result<PremiumFeatures, Error> {
        let tier = load_premium_tier(&env, &user)?;
        Ok(tier.features)
    }

    /// Get the tier level of a user
    pub fn get_tier_level(env: Env, user: Address) -> Result<TierLevel, Error> {
        let tier = load_premium_tier(&env, &user)?;
        Ok(tier.tier)
    }

    /// Get the total contribution amount for a user
    pub fn get_total_contribution(env: Env, user: Address) -> Result<i128, Error> {
        let tier = load_premium_tier(&env, &user)?;
        Ok(tier.contribution)
    }
}

#[cfg(test)]
mod test;