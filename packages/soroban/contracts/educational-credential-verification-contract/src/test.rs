#![cfg(test)]

use crate::{EducatorVerificationContract, EducatorVerificationContractClient, datatype::VerificationLevel};
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, Vec, String
};

// Setup test environment with an admin, a reviewer and an educator
fn setup_test() -> (Env, EducatorVerificationContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(EducatorVerificationContract {}, ());
    let client = EducatorVerificationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let reviewer = Address::generate(&env);
    let educator = Address::generate(&env);

    // Initialize the contract
    client.initialize(&admin);

    // Add a reviewer
    env.mock_all_auths();
    client.add_reviewer(&admin, &reviewer);

    (env, client, admin, reviewer, educator)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(EducatorVerificationContract {}, ());
    let client = EducatorVerificationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Should not allow initializing again - using mock_all_auths to prevent auth failures
    env.mock_all_auths();
    
    // Test that initializing again will fail
    let result = client.try_initialize(&admin);
    assert!(result.is_err());
}

#[test]
fn test_register_educator() {
    let (env, client, _admin, _reviewer, educator) = setup_test();
    
    let name = String::from_str(&env, "John Doe");
    let credentials = Vec::new(&env);
    let specialties = Vec::new(&env);
    
    // Register educator
    env.mock_all_auths();
    
    let registered_address = client.register_educator(&educator, &name, &credentials, &specialties);
    assert_eq!(registered_address, educator);
    
    // Verify that the educator is registered
    let educator_data = client.get_educator(&educator);
    assert!(educator_data.is_some());
    
    let edu = educator_data.unwrap();
    assert_eq!(edu.name, name);
    assert_eq!(edu.verification_status, false);
    assert_eq!(edu.verification_level, VerificationLevel::Pending);
}

#[test]
fn test_add_reviewer() {
    let (env, client, admin, _reviewer, _educator) = setup_test();
    
    let new_reviewer = Address::generate(&env);
    
    // Add a new reviewer
    env.mock_all_auths();
    
    client.add_reviewer(&admin, &new_reviewer);
    
    // Verify that only admins can add reviewers
    let non_admin = Address::generate(&env);
    
    // Test that non-admin can't add a reviewer
    env.mock_all_auths();
    let result = client.try_add_reviewer(&non_admin, &new_reviewer);
    assert!(result.is_err());
}

#[test]
fn test_verify_educator() {
    let (env, client, _admin, reviewer, educator) = setup_test();
    
    // Register an educator
    let name = String::from_str(&env, "John Doe");
    let mut credentials = Vec::new(&env);
    credentials.push_back(String::from_str(&env, "0123456789012345678901234567890123456789012345678901234567890123")); // 64 char hash
    let specialties = Vec::new(&env);
    
    env.mock_all_auths();
    client.register_educator(&educator, &name, &credentials, &specialties);
    
    // Add the credential as verified
    env.mock_all_auths();
    
    // Get the credential from the vector
    let credential = credentials.get_unchecked(0);
    client.add_verified_credential(&reviewer, &credential);
    
    // Verify the educator
    env.mock_all_auths();
    client.verify_educator(&reviewer, &educator, &VerificationLevel::Basic);
    
    // Verify that the educator is now verified
    let educator_data = client.get_educator(&educator).unwrap();
    assert_eq!(educator_data.verification_status, true);
    assert_eq!(educator_data.verification_level, VerificationLevel::Basic);
    assert!(educator_data.nft_token_id.is_some());
}

#[test]
fn test_revoke_verification() {
    // First verify an educator
    let (env, client, admin, reviewer, educator) = setup_test();
    
    // Register an educator
    let name = String::from_str(&env, "John Doe");
    let mut credentials = Vec::new(&env);
    credentials.push_back(String::from_str(&env, "0123456789012345678901234567890123456789012345678901234567890123"));
    let specialties = Vec::new(&env);
    
    env.mock_all_auths();
    client.register_educator(&educator, &name, &credentials, &specialties);
    
    // Add the credential as verified
    env.mock_all_auths();
    
    // Get the credential from the vector
    let credential = credentials.get_unchecked(0);
    client.add_verified_credential(&reviewer, &credential);
    
    // Verify the educator
    env.mock_all_auths();
    client.verify_educator(&reviewer, &educator, &VerificationLevel::Basic);
    
    // Now revoke the verification
    let reason = String::from_str(&env, "Falsified credentials");
    env.mock_all_auths();
    client.revoke_verification(&admin, &educator, &reason);
    
    // Verify that the educator is no longer verified
    let educator_data = client.get_educator(&educator).unwrap();
    assert_eq!(educator_data.verification_status, false);
    assert_eq!(educator_data.verification_level, VerificationLevel::Pending);
    assert!(educator_data.nft_token_id.is_none());
}

#[test]
fn test_submit_review() {
    let (env, client, _admin, reviewer, educator) = setup_test();
    
    // Register an educator
    let name = String::from_str(&env, "John Doe");
    let credentials = Vec::new(&env);
    let specialties = Vec::new(&env);
    
    env.mock_all_auths();
    client.register_educator(&educator, &name, &credentials, &specialties);
    
    // Submit a review
    env.mock_all_auths();
    client.submit_review(&reviewer, &educator, &8);
    
    // Verify that the review was saved correctly
    let reviews = client.get_educator_reviews(&educator);
    assert_eq!(reviews.len(), 1);
    
    // Get the review from vector
    let review = reviews.get_unchecked(0);
    assert_eq!(review.reviewer, reviewer);
    assert_eq!(review.rating, 8);
    
    // Verify that the educator's rating was updated
    let educator_data = client.get_educator(&educator).unwrap();
    assert_eq!(educator_data.rating, 8);
    assert_eq!(educator_data.reviews_count, 1);
}

#[test]
fn test_get_educators_by_specialty() {
    let (env, client, _admin, _reviewer, educator) = setup_test();
    
    // Register an educator with specialties
    let name = String::from_str(&env, "John Doe");
    let credentials = Vec::new(&env);
    let mut specialties = Vec::new(&env);
    specialties.push_back(String::from_str(&env, "Math"));
    specialties.push_back(String::from_str(&env, "Physics"));
    
    env.mock_all_auths();
    client.register_educator(&educator, &name, &credentials, &specialties);
    
    // Register another educator with a different specialty
    let another_educator = Address::generate(&env);
    let another_name = String::from_str(&env, "Jane Smith");
    let mut another_specialties = Vec::new(&env);
    another_specialties.push_back(String::from_str(&env, "Biology"));
    
    env.mock_all_auths();
    client.register_educator(&another_educator, &another_name, &credentials, &another_specialties);
    
    // Search educators by specialty
    let math_educators = client.get_educators_by_specialty(&String::from_str(&env, "Math"));
    assert_eq!(math_educators.len(), 1);
    
    // Compare equality with first item
    assert_eq!(math_educators.get_unchecked(0), educator);
    
    let biology_educators = client.get_educators_by_specialty(&String::from_str(&env, "Biology"));
    assert_eq!(biology_educators.len(), 1);
    assert_eq!(biology_educators.get_unchecked(0), another_educator);
    
    let chemistry_educators = client.get_educators_by_specialty(&String::from_str(&env, "Chemistry"));
    assert_eq!(chemistry_educators.len(), 0);
} 