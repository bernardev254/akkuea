#![cfg(test)]
extern crate std;

use crate::{LearningManagementContract, LearningManagementContractClient};
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env, Vec};

fn create_contract<'a>(env: &Env) -> LearningManagementContractClient<'a> {
    let contract_address = env.register(LearningManagementContract, ());
    LearningManagementContractClient::new(env, &contract_address)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_contract(&env);

    // Initialize contract
    contract.initialize(&admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")] // AlreadyInitialized = 1
fn test_initialize_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_contract(&env);

    contract.initialize(&admin);
    contract.initialize(&admin); // AlreadyInitialized = 1
}

#[test]
fn test_add_and_remove_platform() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let platform = Address::generate(&env);
    let contract = create_contract(&env);

    // Initialize
    contract.initialize(&admin);

    // Add platform
    contract.add_platform(&admin, &platform);

    // Check if platform is authorized
    assert!(contract.is_platform(&platform));

    // Remove platform
    contract.remove_platform(&admin, &platform);

    // Check if platform is no longer authorized
    assert!(!contract.is_platform(&platform));
}

#[test]
fn test_initialize_progress() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let platform = Address::generate(&env);
    let user = Address::generate(&env);
    let contract = create_contract(&env);

    // Initialize contract and add platform
    contract.initialize(&admin);
    contract.add_platform(&admin, &platform);

    // Initialize progress
    let course_id = 1u64;
    let prerequisites = Vec::new(&env);

    let token_id = contract.initialize_progress(&platform, &user, &course_id, &prerequisites);
    assert!(token_id > 0);

    // Verify progress was created
    let progress = contract.get_progress(&token_id);
    assert_eq!(progress.user, user);
    assert_eq!(progress.course_id, course_id);
    assert_eq!(progress.completion_status, 0);
}

#[test]
fn test_update_progress() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let platform = Address::generate(&env);
    let user = Address::generate(&env);
    let contract = create_contract(&env);

    // Setup
    contract.initialize(&admin);
    contract.add_platform(&admin, &platform);

    // Initialize progress
    let course_id = 1u64;
    let prerequisites = Vec::new(&env);
    let token_id = contract.initialize_progress(&platform, &user, &course_id, &prerequisites);

    // Update progress
    let new_status = 50u32;
    contract.update_progress(&platform, &token_id, &new_status);

    // Verify progress was updated
    let progress = contract.get_progress(&token_id);
    assert_eq!(progress.completion_status, new_status);
}

#[test]
fn test_issue_course_nft() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let platform = Address::generate(&env);
    let user = Address::generate(&env);
    let contract = create_contract(&env);

    // Setup
    contract.initialize(&admin);
    contract.add_platform(&admin, &platform);

    // Initialize and complete course
    let course_id = 1u64;
    let token_id = contract.initialize_progress(&platform, &user, &course_id, &Vec::new(&env));

    contract.update_progress(&platform, &token_id, &100u32);

    // Issue NFT
    contract.issue_course_nft(&platform, &token_id);

    // Verify NFT was issued
    let progress = contract.get_progress(&token_id);
    assert!(progress.nft_issued);
}

#[test]
fn test_get_user_nfts() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let platform = Address::generate(&env);
    let user = Address::generate(&env);
    let contract = create_contract(&env);

    // Setup
    contract.initialize(&admin);
    contract.add_platform(&admin, &platform);

    // Create and complete multiple courses
    for course_id in 1..=3 {
        let token_id =
            contract.initialize_progress(&platform, &user, &course_id, &Vec::new(&env));
        contract.update_progress(&platform, &token_id, &100u32);
        contract.issue_course_nft(&platform, &token_id);
    }

    // Get user NFTs
    let nfts = contract.get_user_nfts(&user);
    assert_eq!(nfts.len(), 3);
}


