// #![no_std]
// use soroban_sdk::{
//     contract, contractimpl,
//     Env, Address, BytesN, String,
// };

// #[contract]
// pub struct ReviewContract;

// #[contractimpl]
// impl ReviewContract {
//     pub fn submit_review(env: Env, user: Address, product_id: BytesN<32>, review_text: String) {
//         // Example storage: key = (user, product_id), value = review_text
//         let key = (user, product_id);
//         env.storage().persistent().set(&key, &review_text);
//     }

//     pub fn verify_purchase(_env: Env, _user: Address, _product_id: BytesN<32>) -> bool {
//         // TODO: implement actual verification logic
//         true
//     }
// }





#![no_std]

mod utils;
mod verification;

use soroban_sdk::{contract, contractimpl, Env, Address};
use utils::compose_key;
use verification::verify_purchase;

#[contract]
pub struct ReviewSystem;

#[contractimpl]
impl ReviewSystem {
    // Store a review verification
    pub fn store_verification(env: Env, buyer: Address, content_id: u64) {
        let key = compose_key(&buyer, content_id);
        env.storage().instance().set(&key, &true);
    }

    // Check if a buyer is verified
    pub fn is_verified(env: Env, buyer: Address, content_id: u64) -> bool {
        let key = compose_key(&buyer, content_id);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    // Verify purchase and store verification if valid
    pub fn verify_and_store(env: Env, buyer: Address, content_id: u64) -> bool {
        let valid = verify_purchase(&env, buyer.clone(), content_id);
        if valid {
            Self::store_verification(env.clone(), buyer, content_id);
        }
        valid
    }
}
