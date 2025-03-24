#![no_std]
use soroban_sdk::{
    contract, contractimpl, token, Address, Env, Map, Vec,
};



mod educator_contract {
    soroban_sdk::contractimport!( 
        file = "../../target/wasm32-unknown-unknown/release/educator_verification_nft.wasm"
    );
}
use educator_contract::Client as EducatorClient;
use datatypes::{EducatorTipInfo, TipRecord, Error};
use interfaces:: TippingRewardsInterface;
use crate::educator_contract::Review;



#[contract]
pub struct TippingRewards;


#[contractimpl]
impl TippingRewardsInterface for TippingRewards {

    // Send a tip to an educator
    pub fn send_tip(
        env: &Env,
        token_id: Address,
        from: Address,
        educator: Address,
        amount: i128,
    ) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut educators: Map<Address, EducatorTipInfo> = env.storage().get(&EDUCATORS).unwrap_or(Map::new(&env));
        
        if !educators.contains_key(educator.clone()) {
            return Err(Error::InvalidEducator);
        }

        // Transfer tokens
        let token = token::Client::new(&env, &token_id);
        token.transfer(&from, &educator, &amount);

        // Update educator tip info
        let mut info = educators.get(educator.clone()).unwrap();
        info.total_tips_received += amount;
        educators.set(educator.clone(), info);
        env.storage().set(&EDUCATORS, &educators);

        // Record the tip
        let mut tips_history: Vec<TipRecord> = env.storage().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        tips_history.push_back(TipRecord {
            from: from.clone(),
            to: educator.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        });
        env.storage().set(&TIPS_HISTORY, &tips_history);

        Ok(())
    }

    // Get top educators by reputation
    pub fn get_top_educators(env: &Env, educator_contract: Address, limit: u32) -> Vec<Address> {
        let educators: Map<Address, EducatorTipInfo> = env.storage().get(&EDUCATORS).unwrap_or(Map::new(&env));
        let mut all_educators: Vec<(Address, i128)> = Vec::new(&env);
        
        let educator_client = EducatorClient::new(&env, &educator_contract);
        
        // Get reputation data for each educator
        for (addr, tip_info) in educators.iter() {
            // Get reviews from educator contract
            let reviews = educator_client.get_educator_reviews(&addr);
            let avg_rating = Self::calculate_average_rating(&env, &reviews);
            
            // Combine tips and ratings for ranking
            let reputation_score = Self::combine_reputation_metrics(tip_info.total_tips_received, avg_rating);
            all_educators.push_back((addr, reputation_score));
        }

        // Sort by reputation score (descending)
        all_educators.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Return top N addresses
        let mut result = Vec::new(&env);
        for (addr, _) in all_educators.iter().take(limit as usize) {
            result.push_back(addr.clone());
        }
        
        result
    }

    // Get educator info combining local tip data and educator contract data
    pub fn get_educator_info(
        env: &Env, 
        educator_contract: Address, 
        educator: Address
    ) -> Option<(EducatorTipInfo, Vec<Review>)> {
        let educators: Map<Address, EducatorTipInfo> = env.storage().get(&EDUCATORS).unwrap_or(Map::new(&env));
        let educator_client = EducatorClient::new(&env, &educator_contract);
        
        if let Some(tip_info) = educators.get(educator.clone()) {
            let reviews = educator_client.get_educator_reviews(&educator);
            Some((tip_info, reviews))
        } else {
            None
        }
    }

    // Get tips history for an educator
    pub fn get_educator_tips(env: &Env, educator: Address) -> Vec<TipRecord> {
        let tips_history: Vec<TipRecord> = env.storage().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        let mut educator_tips = Vec::new(&env);
        
        for tip in tips_history.iter() {
            if tip.to == educator {
                educator_tips.push_back(tip.clone());
            }
        }
        
        educator_tips
    }

    // Helper function to calculate average rating from reviews
    fn calculate_average_rating(env: &Env, reviews: &Vec<Review>) -> i128 {
        if reviews.is_empty() {
            return 0;
        }
        
        let mut total = 0_i128;
        for review in reviews.iter() {
            total += review.rating as i128;
        }
        
        total / reviews.len() as i128
    }

    // Helper function to combine different reputation metrics
    fn combine_reputation_metrics(tips: i128, rating: i128) -> i128 {
        // Custom formula to weight both tips and ratings
        // Adjust weights based on your requirements
        (tips / 1000000) + (rating * 100)
    }
}

#[cfg(test)]
mod test;
mod datatypes;
mod interfaces;
