// use soroban_sdk::{Env, Address, Bytes, contracttype, IntoVal};
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ReviewVerification {
//     pub review_id: u64,
//     pub reviewer: Address,
//     pub content_id: u64,
//     pub purchase_verified: bool,
// }

// /// Generate a namespaced key for storing review verifications
// pub fn review_key(env: &Env, review_id: u64) -> Bytes {
//     let mut b = Bytes::new(env);
//     b.extend_from_slice(b"revv_"); // namespace prefix
//     b.extend_from_slice(&review_id.to_le_bytes());
//     b
// }

// /// Register a local purchase in storage (fallback registry)
// pub fn register_local_purchase(env: &Env, reviewer: Address, content_id: u64) {
//     let mut key = Bytes::new(env);

//     // Serialize the Address into Bytes using IntoVal
//     let reviewer_bytes: Bytes = reviewer.clone().into_val(env);
//     key.extend_from_slice(&reviewer_bytes);

//     // Append content_id as bytes
//     key.extend_from_slice(&content_id.to_le_bytes());

//     env.storage().instance().set(&key, &true);
// }


use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Verification(Address, u64), // (buyer_address, content_id)
}

pub fn compose_key(reviewer: &Address, review_id: u64) -> DataKey {
    DataKey::Verification(reviewer.clone(), review_id)
}
