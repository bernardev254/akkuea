use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::{Error, TierAssignmentEvent, TierLevel, TierUpgradeEvent, UserProfile};

/// Event symbol for tier assignment
pub const TIER_ASSIGNED: Symbol = symbol_short!("TIER_ASGN");

/// Event symbol for tier upgrade
pub const TIER_UPGRADED: Symbol = symbol_short!("TIER_UPG");

/// Event symbol for tier downgrade (if allowed in future)
pub const TIER_DOWNGRADED: Symbol = symbol_short!("TIER_DWN");

/// Event symbol for user registration
pub const USER_REGISTERED: Symbol = symbol_short!("USR_REG");

/// Emit a tier assignment event
pub fn emit_tier_assigned(env: &Env, event: &TierAssignmentEvent) -> Result<(), Error> {
    let tier_str = event.tier.to_str();
    
    env.events().publish(
        (TIER_ASSIGNED, symbol_short!("assigned")),
        (
            event.user.clone(),
            String::from_str(env, tier_str),
            event.contribution,
            event.timestamp,
        ),
    );
    
    Ok(())
}

/// Emit a tier upgrade event
pub fn emit_tier_upgraded(env: &Env, event: &TierUpgradeEvent) -> Result<(), Error> {
    let old_tier_str = event.old_tier.to_str();
    let new_tier_str = event.new_tier.to_str();
    
    env.events().publish(
        (TIER_UPGRADED, symbol_short!("upgraded")),
        (
            event.user.clone(),
            String::from_str(env, old_tier_str),
            String::from_str(env, new_tier_str),
            event.contribution,
            event.timestamp,
        ),
    );
    
    Ok(())
}

/// Emit a tier downgrade event
pub fn emit_tier_downgraded(
    env: &Env,
    user: &Address,
    old_tier: &TierLevel,
    new_tier: &TierLevel,
    timestamp: u64,
) -> Result<(), Error> {
    let old_tier_str = old_tier.to_str();
    let new_tier_str = new_tier.to_str();
    
    env.events().publish(
        (TIER_DOWNGRADED, symbol_short!("downgrade")),
        (
            user.clone(),
            String::from_str(env, old_tier_str),
            String::from_str(env, new_tier_str),
            timestamp,
        ),
    );
    
    Ok(())
}

/// Emit a user registration event
pub fn emit_user_registered(env: &Env, profile: &UserProfile) -> Result<(), Error> {
    env.events().publish(
        (USER_REGISTERED, symbol_short!("reg")),
        (
            profile.user.clone(),
            profile.name.clone(),
            profile.preferences.clone(),
            profile.registered_at,
        ),
    );
    Ok(())
}
