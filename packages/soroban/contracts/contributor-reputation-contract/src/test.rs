#![cfg(test)]
extern crate std;

use crate::{ContributorReputation, ContributorReputationClient};
use soroban_sdk::{testutils::Address as _, Address, Env, Map, String};

#[test]
fn test_initialize_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));

    assert_eq!(user_id, 1, "User ID should be 1");
    let areas = contract_client.get_expertise_areas(&user_id);
    assert_eq!(areas.len(), 0);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn test_initialize_user_unauthorized() {
    let env = Env::default();

    let contract_id = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_id);

    let caller = Address::generate(&env);
    let name = String::from_str(&env, "Bob");

    // Try to initialize without authentication
    contract_client.initialize_user(&caller, &name);
}

#[test]
fn test_mint_credential_token_for_verified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user = String::from_str(&env, "Alice");
    let user_id = contract_client.initialize_user(&caller, &user);

    // Verify user first
    let verification_details = String::from_str(&env, "valid credentials");
    contract_client.verify_user(&caller, &user_id, &verification_details);

    // Now mint the credential token
    let token_id = contract_client.mint_credential_token(&caller, &user_id);

    assert_eq!(token_id, 1, "Token ID should be 1");
    let user = contract_client.get_user(&user_id);
    assert_eq!(user.verified, true);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_mint_credential_token_non_existent_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.mint_credential_token(&caller, &999);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_mint_credential_token_for_unverified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.mint_credential_token(&caller, &user_id);
}

#[test]
fn test_update_reputation_for_verified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);
    contract_client.update_reputation(
        &caller,
        &user_id,
        &String::from_str(&env, "Mathematics"),
        &100,
    );

    // Check if the reputation was updated correctly
    let score = contract_client.get_reputation(&user_id, &String::from_str(&env, "Mathematics"));
    assert_eq!(score, 100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_update_reputation_unverified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.update_reputation(
        &caller,
        &user_id,
        &String::from_str(&env, "Mathematics"),
        &100,
    );
}

#[test]
fn test_update_expertise_areas() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    let mut expertise_areas = Map::new(&env);

    expertise_areas.set(String::from_str(&env, "Mathematics"), 5);
    expertise_areas.set(String::from_str(&env, "Physics"), 3);
    contract_client.update_expertise_areas(&caller, &user_id, &expertise_areas);

    let retrieved_areas = contract_client.get_expertise_areas(&user_id);
    assert_eq!(retrieved_areas.len(), 2);
    assert_eq!(
        retrieved_areas
            .get(String::from_str(&env, "Mathematics"))
            .unwrap(),
        5
    );
    assert_eq!(
        retrieved_areas
            .get(String::from_str(&env, "Physics"))
            .unwrap(),
        3
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_update_expertise_areas_non_existent_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let expertise_areas = Map::new(&env);
    contract_client.update_expertise_areas(&caller, &999, &expertise_areas);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_reverify_verified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
}

#[test]
fn test_verify_content() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "Mathematics"), 5);
    contract_client.update_expertise_areas(&caller, &user_id, &expertise_areas);

    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));

    assert!(true, "Content verification completed without errors");
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_verify_content_unverified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_verify_content_no_expertise() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.mint_credential_token(&caller, &user_id);
    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")]
fn test_get_reputation_non_existent() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.get_reputation(&user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_get_expertise_areas_non_existent_user() {
    let env = Env::default();
    let _caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.get_expertise_areas(&999);
}

#[test]
fn test_verify_user_success() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();

    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    let user = contract_client.get_user(&user_id);
    assert!(user.verified, "User should be verified");
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_verify_user_rejection() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();

    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));

    // Attempt to verify without valid or empty details
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, ""));
}

#[test]
fn test_multiple_tokens_mint_same_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    // Attempt to remint the token for the same user
    contract_client.mint_credential_token(&caller, &user_id);
}
