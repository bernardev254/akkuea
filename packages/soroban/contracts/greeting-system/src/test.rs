#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

// Helper function to create a test environment
fn create_test_env() -> Env {
    Env::default()
}

// Helper function to register the contract
fn register_contract(env: &Env) -> Address {
    env.register(GreetingSystemContract, ())
}

// Helper function to create a client
fn create_client<'a>(env: &'a Env, contract_id: &'a Address) -> GreetingSystemContractClient<'a> {
    GreetingSystemContractClient::new(env, contract_id)
}

#[test]
fn test_initialize() {
    let env = create_test_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    // Verify greeting count is 0
    assert_eq!(client.get_greeting_count(), 0);
}

#[test]
fn test_create_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let message = String::from_str(&env, "Hello, Akkuea!");

    let greeting_id = client.create_greeting(&creator, &message);

    assert_eq!(greeting_id, 1);
    assert_eq!(client.get_greeting_count(), 1);

    // Verify greeting data
    let greeting = client.get_greeting(&greeting_id);
    assert_eq!(greeting.id, 1);
    assert_eq!(greeting.creator, creator);
    assert_eq!(greeting.message, message);
}

#[test]
fn test_create_multiple_greetings() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator1 = Address::generate(&env);
    let creator2 = Address::generate(&env);

    let id1 = client.create_greeting(&creator1, &String::from_str(&env, "First greeting"));
    let id2 = client.create_greeting(&creator2, &String::from_str(&env, "Second greeting"));
    let id3 = client.create_greeting(&creator1, &String::from_str(&env, "Third greeting"));

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    assert_eq!(client.get_greeting_count(), 3);
}

#[test]
#[should_panic(expected = "Message cannot be empty")]
fn test_create_greeting_empty_message() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let empty_message = String::from_str(&env, "");

    client.create_greeting(&creator, &empty_message);
}

#[test]
#[should_panic(expected = "Message too long")]
fn test_create_greeting_too_long() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    // Create a message longer than 1000 characters
    let long_message = String::from_str(&env, &"a".repeat(1001));

    client.create_greeting(&creator, &long_message);
}

#[test]
fn test_greeting_exists() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    assert!(client.greeting_exists(&greeting_id));
    assert!(!client.greeting_exists(&999));
}

#[test]
#[should_panic(expected = "Greeting not found")]
fn test_get_nonexistent_greeting() {
    let env = create_test_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    client.get_greeting(&999);
}

// ========== Like Tests ==========

#[test]
fn test_like_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let liker = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Like me!"));

    // Like the greeting
    let like_count = client.like_greeting(&greeting_id, &liker);

    assert_eq!(like_count, 1);
    assert_eq!(client.get_like_count(&greeting_id), 1);
    assert!(client.has_user_liked(&greeting_id, &liker));
}

#[test]
fn test_multiple_users_like_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Popular!"));

    let liker1 = Address::generate(&env);
    let liker2 = Address::generate(&env);
    let liker3 = Address::generate(&env);

    client.like_greeting(&greeting_id, &liker1);
    client.like_greeting(&greeting_id, &liker2);
    client.like_greeting(&greeting_id, &liker3);

    assert_eq!(client.get_like_count(&greeting_id), 3);
    assert!(client.has_user_liked(&greeting_id, &liker1));
    assert!(client.has_user_liked(&greeting_id, &liker2));
    assert!(client.has_user_liked(&greeting_id, &liker3));
}

#[test]
#[should_panic(expected = "User has already liked this greeting")]
fn test_duplicate_like() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let liker = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    client.like_greeting(&greeting_id, &liker);
    // Try to like again - should panic
    client.like_greeting(&greeting_id, &liker);
}

#[test]
#[should_panic(expected = "Greeting not found")]
fn test_like_nonexistent_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let liker = Address::generate(&env);
    client.like_greeting(&999, &liker);
}

// ========== Comment Tests ==========

#[test]
fn test_comment_on_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Comment on me!"));

    let comment_text = String::from_str(&env, "Great greeting!");
    let comment_count = client.comment_on_greeting(&greeting_id, &commenter, &comment_text);

    assert_eq!(comment_count, 1);
    assert_eq!(client.get_comment_count(&greeting_id), 1);

    let comments = client.get_comments(&greeting_id);
    assert_eq!(comments.len(), 1);
    assert_eq!(comments.get(0).unwrap().user, commenter);
    assert_eq!(comments.get(0).unwrap().comment_text, comment_text);
}

#[test]
fn test_multiple_comments() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Discussion topic"));

    let commenter1 = Address::generate(&env);
    let commenter2 = Address::generate(&env);
    let commenter3 = Address::generate(&env);

    client.comment_on_greeting(&greeting_id, &commenter1, &String::from_str(&env, "First!"));
    client.comment_on_greeting(&greeting_id, &commenter2, &String::from_str(&env, "Second!"));
    client.comment_on_greeting(&greeting_id, &commenter3, &String::from_str(&env, "Third!"));

    assert_eq!(client.get_comment_count(&greeting_id), 3);

    let comments = client.get_comments(&greeting_id);
    assert_eq!(comments.len(), 3);
}

#[test]
#[should_panic(expected = "Comment cannot be empty")]
fn test_empty_comment() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, ""));
}

#[test]
#[should_panic(expected = "Comment too long")]
fn test_comment_too_long() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // Create a comment longer than 500 characters
    let long_comment = String::from_str(&env, &"a".repeat(501));
    client.comment_on_greeting(&greeting_id, &commenter, &long_comment);
}

#[test]
#[should_panic(expected = "Greeting not found")]
fn test_comment_on_nonexistent_greeting() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let commenter = Address::generate(&env);
    client.comment_on_greeting(&999, &commenter, &String::from_str(&env, "Comment"));
}

#[test]
fn test_same_user_multiple_comments() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // Same user can comment multiple times
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "First comment"));
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "Second comment"));

    assert_eq!(client.get_comment_count(&greeting_id), 2);
}

// ========== Integration Tests ==========

#[test]
fn test_full_workflow() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    // Create a greeting
    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Hello World!"));

    // Multiple users like it
    let liker1 = Address::generate(&env);
    let liker2 = Address::generate(&env);
    client.like_greeting(&greeting_id, &liker1);
    client.like_greeting(&greeting_id, &liker2);

    // Multiple users comment
    let commenter1 = Address::generate(&env);
    let commenter2 = Address::generate(&env);
    client.comment_on_greeting(&greeting_id, &commenter1, &String::from_str(&env, "Nice!"));
    client.comment_on_greeting(&greeting_id, &commenter2, &String::from_str(&env, "Awesome!"));

    // Verify final state
    assert_eq!(client.get_like_count(&greeting_id), 2);
    assert_eq!(client.get_comment_count(&greeting_id), 2);
    assert!(client.has_user_liked(&greeting_id, &liker1));
    assert!(client.has_user_liked(&greeting_id, &liker2));
}

#[test]
fn test_multiple_greetings_with_interactions() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let user = Address::generate(&env);

    // Create multiple greetings
    let id1 = client.create_greeting(&creator, &String::from_str(&env, "First"));
    let id2 = client.create_greeting(&creator, &String::from_str(&env, "Second"));

    // Interact with both
    client.like_greeting(&id1, &user);
    client.like_greeting(&id2, &user);
    client.comment_on_greeting(&id1, &user, &String::from_str(&env, "Comment on first"));
    client.comment_on_greeting(&id2, &user, &String::from_str(&env, "Comment on second"));

    // Verify each greeting has independent state
    assert_eq!(client.get_like_count(&id1), 1);
    assert_eq!(client.get_like_count(&id2), 1);
    assert_eq!(client.get_comment_count(&id1), 1);
    assert_eq!(client.get_comment_count(&id2), 1);
}

#[test]
fn test_get_comments_empty() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "No comments yet"));

    let comments = client.get_comments(&greeting_id);
    assert_eq!(comments.len(), 0);
}

#[test]
fn test_like_count_zero_initially() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "No likes yet"));

    assert_eq!(client.get_like_count(&greeting_id), 0);
}

// ========== Security Tests ==========

#[test]
#[should_panic(expected = "Comment contains inappropriate content")]
fn test_profanity_filter_spam() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // Try to post a comment with blacklisted word "spam"
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "spam"));
}

#[test]
#[should_panic(expected = "Comment contains inappropriate content")]
fn test_profanity_filter_scam() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // Try to post a comment with blacklisted word "scam"
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "scam"));
}

#[test]
#[should_panic(expected = "Comment contains inappropriate content")]
fn test_profanity_filter_hack() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // Try to post a comment with blacklisted word "hack"
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "hack"));
}

#[test]
fn test_profanity_filter_allows_clean_content() {
    let env = create_test_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let creator = Address::generate(&env);
    let commenter = Address::generate(&env);
    let greeting_id = client.create_greeting(&creator, &String::from_str(&env, "Test"));

    // These should all pass the profanity filter
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "Great post!"));
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "Very informative"));
    client.comment_on_greeting(&greeting_id, &commenter, &String::from_str(&env, "Thanks for sharing"));

    assert_eq!(client.get_comment_count(&greeting_id), 3);
}


