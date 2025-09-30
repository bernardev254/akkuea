#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec};

mod social;
mod utils;

#[cfg(test)]
mod test;

use social::{SocialInteraction, like_greeting, comment_on_greeting, get_like_count, get_comments};

/// Represents a greeting in the system
#[derive(Clone)]
#[contracttype]
pub struct Greeting {
    pub id: u64,
    pub creator: Address,
    pub message: String,
    pub timestamp: u64,
}

// Main contract structure for the greeting system
#[contract]
pub struct GreetingSystemContract;

#[contractimpl]
impl GreetingSystemContract {
    // Initialize the contract
    // Sets up the initial greeting counter
    pub fn initialize(env: Env) {
        // Initialize greeting counter to 0
        env.storage().persistent().set(&symbol_short!("GRT_CNT"), &0u64);
    }

    /// Create a new greeting
    pub fn create_greeting(env: Env, creator: Address, message: String) -> u64 {
        // Require authentication from the creator
        creator.require_auth();

        // Validate message is not empty
        if message.len() == 0 {
            panic!("Message cannot be empty");
        }

        // Validate message length (max 1000 characters)
        if message.len() > 1000 {
            panic!("Message too long (max 1000 characters)");
        }

        // Get and increment greeting counter
        let mut counter: u64 = env
            .storage()
            .persistent()
            .get(&symbol_short!("GRT_CNT"))
            .unwrap_or(0);

        // Check for integer overflow before incrementing
        if counter == u64::MAX {
            panic!("Maximum number of greetings reached");
        }

        counter += 1;

        // Create the greeting
        let greeting = Greeting {
            id: counter,
            creator: creator.clone(),
            message: message.clone(),
            timestamp: env.ledger().timestamp(),
        };

        // Store the greeting using a tuple key (symbol, u64)
        env.storage().persistent().set(&(symbol_short!("GRT"), counter), &greeting);

        // Update the counter
        env.storage().persistent().set(&symbol_short!("GRT_CNT"), &counter);

        // Emit event
        env.events().publish(
            (symbol_short!("grt_crtd"),),
            (counter, creator, message, env.ledger().timestamp()),
        );

        counter
    }

    // Get a greeting by ID
    pub fn get_greeting(env: Env, greeting_id: u64) -> Greeting {
        env.storage()
            .persistent()
            .get(&(symbol_short!("GRT"), greeting_id))
            .unwrap_or_else(|| panic!("Greeting not found"))
    }

    // Check if a greeting exists
    pub fn greeting_exists(env: Env, greeting_id: u64) -> bool {
        env.storage().persistent().has(&(symbol_short!("GRT"), greeting_id))
    }

    // Get the total number of greetings
    pub fn get_greeting_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&symbol_short!("GRT_CNT"))
            .unwrap_or(0)
    }

    // Like a greeting
    pub fn like_greeting(env: Env, greeting_id: u64, user: Address) -> u64 {
        like_greeting(env, greeting_id, user)
    }

    // Comment on a greeting
    // Returns the new comment count
    pub fn comment_on_greeting(env: Env, greeting_id: u64, user: Address, text: String) -> u64 {
        comment_on_greeting(env, greeting_id, user, text)
    }

    // Get the like count for a greeting
    pub fn get_like_count(env: Env, greeting_id: u64) -> u64 {
        get_like_count(env, greeting_id)
    }

    // Get all comments for a greeting
    pub fn get_comments(env: Env, greeting_id: u64) -> Vec<SocialInteraction> {
        get_comments(env, greeting_id)
    }

    // Get the comment count for a greeting
    pub fn get_comment_count(env: Env, greeting_id: u64) -> u64 {
        let comments = get_comments(env, greeting_id);
        comments.len() as u64
    }

    // Check if a user has liked a greeting
    pub fn has_user_liked(env: Env, greeting_id: u64, user: Address) -> bool {
        social::has_user_liked(env, greeting_id, user)
    }
}

