#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Address, String, Vec};

#[test]
fn test_initialize_user() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    let user_id = client.initialize_user(&user_address, &name);
    assert_eq!(user_id, 1);

    // Test duplicate initialization
    let result = client.try_initialize_user(&user_address, &name);
    assert!(result.is_err());
}

#[test]
fn test_get_reputation() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.user, user_address);
    assert_eq!(reputation.score, 50); // Initial neutral score
    assert_eq!(reputation.projects_completed, 0);
    assert_eq!(reputation.milestones_missed, 0);
    assert_eq!(reputation.total_projects, 0);
}

#[test]
fn test_update_reputation_success() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Update reputation for successful project
    client.update_reputation(&admin_address, &user_address, &1, &true);

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.score, 60); // 50 + 10 for success
    assert_eq!(reputation.projects_completed, 1);
    assert_eq!(reputation.total_projects, 1);
}

#[test]
fn test_update_reputation_failure() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Update reputation for failed project
    client.update_reputation(&admin_address, &user_address, &1, &false);

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.score, 45); // 50 - 5 for failure
    assert_eq!(reputation.projects_completed, 0);
    assert_eq!(reputation.total_projects, 1);
}

#[test]
fn test_get_voting_power() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Initial voting power (score 50)
    let voting_power = client.get_voting_power(&user_address);
    assert_eq!(voting_power, 6); // 1 + (50 / 10) = 6

    // Increase reputation and check voting power
    client.update_reputation(&admin_address, &user_address, &1, &true);
    let voting_power = client.get_voting_power(&user_address);
    assert_eq!(voting_power, 7); // 1 + (60 / 10) = 7
}

#[test]
fn test_penalize_missed_milestone() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Apply penalty for missed milestone
    client.penalize_missed_milestone(&admin_address, &user_address, &1);

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.score, 35); // 50 - 15 penalty
    assert_eq!(reputation.milestones_missed, 1);
}

#[test]
fn test_vote_for_project() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let voter_address = Address::generate(&env);
    let name = String::from_str(&env, "Test Voter");

    env.mock_all_auths();

    client.initialize_user(&voter_address, &name);

    // Vote for project
    let voting_power = client.vote_for_project(&voter_address, &1);
    assert_eq!(voting_power, 6); // Initial voting power

    // Check project voting power
    let project_voting_power = client.get_project_voting_power(&1);
    assert_eq!(project_voting_power, 6);

    // Test duplicate vote
    let result = client.try_vote_for_project(&voter_address, &1);
    assert!(result.is_err());
}

#[test]
fn test_multiple_voters() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);
    let name = String::from_str(&env, "Test Voter");

    env.mock_all_auths();

    client.initialize_user(&voter1, &name);
    client.initialize_user(&voter2, &name);
    client.initialize_user(&voter3, &name);

    // All voters vote for the same project
    client.vote_for_project(&voter1, &1);
    client.vote_for_project(&voter2, &1);
    client.vote_for_project(&voter3, &1);

    // Check total voting power (3 * 6 = 18)
    let project_voting_power = client.get_project_voting_power(&1);
    assert_eq!(project_voting_power, 18);

    // Get voters with their voting power
    let voters = client.get_project_voters(&1);
    assert_eq!(voters.len(), 3);
    assert_eq!(voters.get(voter1), Some(6));
    assert_eq!(voters.get(voter2), Some(6));
    assert_eq!(voters.get(voter3), Some(6));
}

#[test]
fn test_project_approval() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    // Create multiple voters with high reputation
    let mut voters = Vec::new(&env);
    let name = String::from_str(&env, "Test Voter");

    env.mock_all_auths();

    // Create 20 voters (each with voting power 6 = 120 total, above 100 threshold)
    for _i in 0..20 {
        let voter = Address::generate(&env);
        client.initialize_user(&voter, &name);
        voters.push_back(voter);
    }

    // All voters vote for the same project
    for voter in voters.iter() {
        client.vote_for_project(&voter, &1);
    }

    // Check total voting power (20 * 6 = 120)
    let project_voting_power = client.get_project_voting_power(&1);
    assert_eq!(project_voting_power, 120);
}

#[test]
fn test_complete_milestone() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let creator_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test Creator");

    env.mock_all_auths();

    client.initialize_user(&creator_address, &name);

    // Complete milestone
    client.complete_milestone(&admin_address, &1, &1, &creator_address);

    let reputation = client.get_reputation(&creator_address);
    assert_eq!(reputation.score, 60); // 50 + 10 for successful completion
    assert_eq!(reputation.projects_completed, 1);
}

#[test]
fn test_reputation_stats() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user1, &name);
    client.initialize_user(&user2, &name);

    // Update reputations
    client.update_reputation(&admin, &user1, &1, &true); // 50 -> 60
    client.update_reputation(&admin, &user2, &1, &false); // 50 -> 45
    client.penalize_missed_milestone(&admin, &user1, &1); // 60 -> 45

    let stats = client.get_reputation_stats();
    assert_eq!(stats.total_users, 2);
    assert_eq!(stats.average_reputation, 45); // (45 + 45) / 2
    assert_eq!(stats.total_projects_completed, 1);
    assert_eq!(stats.total_milestones_missed, 1);
    assert_eq!(stats.highest_reputation, 45);
    assert_eq!(stats.lowest_reputation, 45);
}

#[test]
fn test_reputation_underflow_protection() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Try to apply multiple penalties that would cause underflow
    for _ in 0..4 {
        let result = client.try_penalize_missed_milestone(&admin_address, &user_address, &1);
        if result.is_err() {
            break; // Should fail on 4th penalty (50 - 4*15 = -10)
        }
    }

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.score, 5); // 50 - 3*15 = 5 (last penalty should fail)
}

#[test]
fn test_reputation_overflow_protection() {
    let env = Env::default();
    let contract_id = env.register(MilestoneFinance, ());
    let client = MilestoneFinanceClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let admin_address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");

    env.mock_all_auths();

    client.initialize_user(&user_address, &name);

    // Apply many successful updates (should cap at 100)
    for i in 0..10 {
        client.update_reputation(&admin_address, &user_address, &i, &true);
    }

    let reputation = client.get_reputation(&user_address);
    assert_eq!(reputation.score, 100); // Capped at maximum
    assert_eq!(reputation.projects_completed, 10);
}
