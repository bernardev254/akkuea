use soroban_sdk::{Address, Env, String, symbol_short};

/// Custom error types for the milestone finance contract
#[derive(Debug, Clone)]
pub enum Error {
    UserNotFound,
    UserAlreadyExists,
    ProjectNotFound,
    MilestoneNotFound,
    InsufficientReputation,
    InvalidVotingPower,
    DuplicateVote,
    Unauthorized,
    InvalidReputationScore,
    ReputationUnderflow,
    ReputationOverflow,
}

/// Calculate voting power based on reputation score
/// Formula: voting_power = base_power + (reputation_score / 10)
/// This gives higher reputation users more voting influence
pub fn calculate_voting_power(reputation_score: u32) -> u32 {
    const BASE_VOTING_POWER: u32 = 1;
    const REPUTATION_MULTIPLIER: u32 = 10;
    
    BASE_VOTING_POWER + (reputation_score / REPUTATION_MULTIPLIER)
}

/// Calculate reputation change based on project success/failure
/// Success: +10 points, Failure: -5 points
pub fn calculate_reputation_change(success: bool) -> i32 {
    if success {
        10
    } else {
        -5
    }
}

/// Calculate penalty for missed milestones
/// Penalty: -15 points per missed milestone
pub fn calculate_milestone_penalty() -> i32 {
    -15
}

/// Emit reputation update event
pub fn emit_reputation_event(
    env: &Env,
    user: Address,
    old_score: u32,
    new_score: u32,
    reason: String,
) {
    env.events().publish(
        (symbol_short!("rep_upd"), user),
        (old_score, new_score, reason),
    );
}

/// Emit voting event
pub fn emit_voting_event(
    env: &Env,
    voter: Address,
    project_id: u64,
    voting_power: u32,
) {
    env.events().publish(
        (symbol_short!("vote_cast"), voter),
        (project_id, voting_power),
    );
}

/// Emit milestone completion event
pub fn emit_milestone_event(
    env: &Env,
    project_id: u64,
    milestone_id: u64,
    creator: Address,
    success: bool,
) {
    env.events().publish(
        (symbol_short!("milestone"), creator),
        (project_id, milestone_id, success),
    );
}
