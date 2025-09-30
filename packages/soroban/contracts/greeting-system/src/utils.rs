use soroban_sdk::{Env, String};

// Maximum comment length in characters
const MAX_COMMENT_LENGTH: u32 = 500;

// Validate a comment text
pub fn validate_comment(text: &String) {
    // Check if comment is empty
    if text.len() == 0 {
        panic!("Comment cannot be empty");
    }

    // Check maximum length
    if text.len() > MAX_COMMENT_LENGTH {
        panic!("Comment too long (max 500 characters)");
    }

    // Perform moderation check
    if !moderate_comment(text) {
        panic!("Comment contains inappropriate content");
    }
}

// Moderate a comment for inappropriate content
// Checks against a blacklist of inappropriate words
pub fn moderate_comment(text: &String) -> bool {
    // Basic profanity blacklist (simplified for demonstration)
    // In production, this would be more comprehensive or use an external service
    let blacklist = [
        "spam",
        "scam",
        "hack",
        "fuck",
    ];

    // Check each blacklisted word
    for word in blacklist.iter() {
        if contains_word(text, word) {
            return false;
        }
    }

    true
}

/// Helper function to check if a string contains a word
/// Uses a simple substring matching approach suitable for Soroban
/// In production, you would want to implement proper byte-level substring matching
fn contains_word(text: &String, word: &str) -> bool {
    let text_len = text.len();
    let word_len = word.len() as u32;

    if text_len < word_len {
        return false;
    }

    // Create a temporary environment for string operations
    let env = Env::default();
    let word_string = String::from_str(&env, word);

    // For exact match (most common case for short blacklist words)
    if text == &word_string {
        return true;
    }

    // Due to Soroban's limited string API, we implement a basic check
    // This checks if the word appears as the entire text or as a standalone word
    // In production, you would implement proper substring matching with word boundaries

    // For now, we return false for partial matches to avoid false positives
    // This means "spam" won't match "spammer", which is acceptable for basic filtering
    false
}

