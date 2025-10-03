use soroban_sdk::{Env, Address, Symbol, symbol_short, Vec, IntoVal};

use crate::DataKey;

/// Call the external verification service to check if a buyer can review
pub fn verify_review_eligibility(env: &Env, buyer: Address, content_id: Symbol) -> bool {
    let service: Address = env
        .storage()
        .persistent()
        .get(&DataKey::VerificationService)
        .expect("Verification service not set");

    // Function symbol expected in the external contract
    let fn_sym = symbol_short!("verify");

    // Build args (buyer, content_id)
    let args: Vec<_> = vec![
        buyer.into_val(env),
        content_id.into_val(env),
    ];

    // Call external verification contract (returns `bool`)
    env.invoke_contract::<bool>(&service, &fn_sym, args)
}