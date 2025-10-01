use soroban_sdk::{
    contracttype, Address, Env, Bytes, BytesN, Map, Symbol, Vec, IntoVal,
};

use crate::DataKey;

/// Event symbols (reused in code)
pub const VERIFICATION_SUCCESS_EVENT: Symbol = Symbol::short("VerificationSuccess");
pub const VERIFICATION_FAILED_EVENT: Symbol = Symbol::short("VerificationFailed");

/// A compact, contract-compatible struct stored in contract storage per review
#[contracttype]
#[derive(Clone)]
pub struct ReviewVerification {
    pub review_id: u64,
    pub reviewer: Address,
    pub content_id: u64,
    pub purchase_verified: bool,
}

/// Storage key namespace helper
pub struct StorageKeys;

impl StorageKeys {
    pub fn review_key(env: &Env, review_id: u64) -> Bytes {
        // Compose a namespaced key: ("review_ver", review_id)
        let mut v = Vec::new(env);
        v.push_back(Bytes::from_slice(env, b"review_ver").into_val(env));
        v.push_back(review_id.into_val(env));
        // If you want to create a Bytes key:
        let mut b = Bytes::new(env);
        b.append(&Bytes::from_slice(env, b"review_ver"));
        b.append(&Bytes::from_slice(env, &review_id.to_le_bytes()));
        b
    }
}

/// Use tuple (Symbol, u64) as storage key for reviews.
///
/// set_review_verification
pub fn set_review_verification(env: &Env, review_id: u64, vr: &ReviewVerification) {
    let key_sym = Symbol::new(env, "review_ver");
    env.storage().set(&(key_sym, review_id), vr);
}

/// read_review_verification
pub fn read_review_verification(env: &Env, review_id: u64) -> Option<ReviewVerification> {
    let key_sym = Symbol::new(env, "review_ver");
    env.storage().get(&(key_sym, review_id))
}

/// Fallback local purchase registry helpers.
/// In a real setup you probably won't keep local purchases here,
/// but the contract exposes these for unit tests or on-chain fallback.

/// Registers a local purchase (admin or payment contract would call this in real life)
pub fn register_local_purchase(env: &Env, buyer: &Address, content_id: u64) {
    let key_sym = Symbol::new(env, "local_purchase");
    // Use (buyer, content_id) tuple to store a boolean
    env.storage().set(&(key_sym, buyer.clone(), content_id), &true);
}

/// Checks whether there's a recorded local purchase
pub fn has_local_purchase(env: &Env, buyer: &Address, content_id: u64) -> bool {
    let key_sym = Symbol::new(env, "local_purchase");
    env.storage()
        .get(&(key_sym, buyer.clone(), content_id))
        .unwrap_or(false)
}

/// Helper to build a ReviewVerification key (not strictly necessary, exported for clarity)
pub fn get_review_key(_env: &Env, review_id: u64) -> (Symbol, u64) {
    (Symbol::new(_env, "review_ver"), review_id)
}