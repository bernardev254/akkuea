#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Map};

mod reputation;
mod utils;

pub use reputation::*;
pub use utils::*;

#[cfg(test)]
mod test;

#[contract]
pub struct MilestoneFinance;

#[contractimpl]
impl MilestoneFinance {
    /// Initialize a new user in the reputation system
    pub fn initialize_user(env: Env, caller: Address, name: String) -> u64 {
        caller.require_auth();
        reputation::initialize_user(env, caller, name)
    }

    /// Update reputation based on project or milestone outcomes
    pub fn update_reputation(
        env: Env,
        caller: Address,
        user: Address,
        project_id: u64,
        success: bool,
    ) {
        caller.require_auth();
        reputation::update_reputation(env, caller, user, project_id, success)
    }

    /// Get voting power based on reputation score
    pub fn get_voting_power(env: Env, user: Address) -> u32 {
        reputation::get_voting_power(env, user)
    }

    /// Apply reputation penalty for missed milestones
    pub fn penalize_missed_milestone(
        env: Env,
        caller: Address,
        user: Address,
        milestone_id: u64,
    ) {
        caller.require_auth();
        reputation::penalize_missed_milestone(env, caller, user, milestone_id)
    }

    /// Get user reputation details
    pub fn get_reputation(env: Env, user: Address) -> Reputation {
        reputation::get_reputation(env, user)
    }

    /// Vote for a project with reputation-based voting power
    pub fn vote_for_project(
        env: Env,
        voter: Address,
        project_id: u64,
    ) -> u32 {
        voter.require_auth();
        reputation::vote_for_project(env, voter, project_id)
    }

    /// Get total voting power for a project
    pub fn get_project_voting_power(env: Env, project_id: u64) -> u32 {
        reputation::get_project_voting_power(env, project_id)
    }

    /// Get project voters with their voting power
    pub fn get_project_voters(env: Env, project_id: u64) -> Map<Address, u32> {
        reputation::get_project_voters(env, project_id)
    }

    /// Complete a milestone and update creator reputation
    pub fn complete_milestone(
        env: Env,
        caller: Address,
        project_id: u64,
        milestone_id: u64,
        creator: Address,
    ) {
        caller.require_auth();
        reputation::complete_milestone(env, caller, project_id, milestone_id, creator)
    }

    /// Get reputation statistics for analytics
    pub fn get_reputation_stats(env: Env) -> ReputationStats {
        reputation::get_reputation_stats(env)
    }
}
