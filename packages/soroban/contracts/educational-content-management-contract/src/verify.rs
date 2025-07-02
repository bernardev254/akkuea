use soroban_sdk::{Address, Env, symbol_short};
use crate::storage::{get_content, save_content, VerificationLevel};

// Verify educational content with different tiers
pub fn verify_content(
    env: &Env,
    content_id: u64,
    verifier: Address,
    level: VerificationLevel,
) -> VerificationLevel {
    let mut content = get_content(env, content_id);

    // Prevent overwriting a higher verification level with a lower one
    if level <= content.verification_level {
        panic!("cannot overwrite a higher or equal verification level");
    }

    // Update the verification level
    content.verification_level = level;

    save_content(env, &content);

    // Emit verification event with the new level
    env.events().publish(
        (symbol_short!("VERIFY"),),
        (
            content_id,
            verifier,
            level,
        ),
    );

    content.verification_level
}