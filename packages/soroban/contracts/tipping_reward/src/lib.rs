#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol, Vec
};
use soroban_sdk::token::Client as TokenClient;

mod educator_contract {
    soroban_sdk::contractimport!( 
        file = "../../target/wasm32-unknown-unknown/release/educator_verification_nft.wasm"
    );
}
use educator_contract::Client as EducatorClient;
use datatypes::{TipRecord, Error, TIPS_HISTORY};
use interfaces::TippingRewardsInterface;



#[contract]
pub struct TippingRewards;

const EDUCATOR_CONTRACT: Symbol = symbol_short!("EDU_CONT");

#[contractimpl]
impl TippingRewardsInterface for TippingRewards {

    // Constructor to save the educator-verification-nft contract address on deployment
    fn __constructor(env: &Env, educator_contract: Address) {
        env.storage().instance().set(&EDUCATOR_CONTRACT, &educator_contract);
    }

    // Send a tip to an educator
    fn send_tip(
        env: &Env,
        token: Address,
        from: Address,
        educator: Address,
        amount: i128,
    ) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Check if educator exists in the educator contract
        let educator_client = EducatorClient::new(&env, &env.storage().instance().get(&EDUCATOR_CONTRACT).unwrap());
        educator_client.get_educator(&educator)
            .ok_or(Error::InvalidEducator)?;

        // Transfer tokens
        let token = TokenClient::new(&env, &token);
        token.transfer(&from, &educator, &amount);

        // Record the tip
        let mut tips_history: Vec<TipRecord> = env.storage().instance().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        tips_history.push_back(TipRecord {
            from: from.clone(),
            to: educator.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        });
        env.storage().instance().set(&TIPS_HISTORY, &tips_history);

        // Publish event
        env.events().publish(
            (TIPS_HISTORY, symbol_short!("send")),
            (from, educator, amount)
        );

        Ok(())
    }

    
    // Get top educators by reputation
    fn get_top_educators(
        env: &Env, 
        limit: u32
    ) -> Vec<educator_contract::Educator> {
        // Get educator contract address from storage
        let educator_contract = env.storage().instance().get(&EDUCATOR_CONTRACT).unwrap();
        let educator_client = EducatorClient::new(&env, &educator_contract);
        
        // Get all educators from educator contract
        let all_educators = educator_client.get_verified_educators();
        let mut educator_scores: Vec<(Address, i128)> = Vec::new(&env);
        
        // Calculate tip amounts for each educator
        for educator in all_educators.iter() {
            // Get tips history
            let tips_history: Vec<TipRecord> = env.storage().instance().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
            let mut total_tips = 0_i128;
            
            // Sum up all tips for this educator
            for tip in tips_history.iter() {
                if tip.to == educator {
                    total_tips += tip.amount;
                }
            }
            
            // Get reviews from educator contract
            let reviews = educator_client.get_educator_reviews(&educator);
           
            let mut ratings = Vec::new(&env);
            for review in reviews.iter() {
                ratings.push_back(review.rating);
            }
            let avg_rating = Self::calculate_average_rating(ratings);
            
            // Calculate reputation score
            let reputation_score = Self::combine_reputation_metrics(total_tips, avg_rating);
            
            educator_scores.push_back((educator.clone(), reputation_score));
        }
        
        // Sort by reputation score (descending)
        for _i in 0..educator_scores.len() {
            for j in 0..educator_scores.len() - 1 {
                if educator_scores.get_unchecked(j).1 < educator_scores.get_unchecked(j + 1).1 {
                    let temp = educator_scores.get_unchecked(j);
                    educator_scores.set(j, educator_scores.get_unchecked(j + 1));
                    educator_scores.set(j + 1, temp);
                }
            }
        }
        
        // Return top N educators
        let mut result = Vec::new(&env);
        for (addr, _) in educator_scores.iter().take(limit as usize) {
            if let Some(educator) = educator_client.get_educator(&addr) {
                result.push_back(educator);
            }
        }
        
        // Publish event with top educators
        env.events().publish(
            (symbol_short!("TOP_EDU"), symbol_short!("list")),
            (limit, result.clone())
        );
        
        result
    }


    // Helper function to calculate average rating from reviews
    fn calculate_average_rating(ratings: Vec<u32>) -> i128 {
        if ratings.len() == 0 {
            return 0;
        }
        
        let mut total = 0_i128;
        for i in 0..ratings.len() {
            total += ratings.get_unchecked(i) as i128;
        }
        total / (ratings.len() as i128)
    }


    // Helper function to combine different reputation metrics
    fn combine_reputation_metrics(total_tips: i128, avg_rating: i128) -> i128 {
        // Weight factors (adjustable)
        let tip_weight = 1;      // Base weight for tips
        let rating_weight = 50;  // Higher weight for ratings (1-5 scale)
        
        // Use raw tip amount without assumptions about decimal places
        let weighted_tips = total_tips * tip_weight;
        let weighted_rating = avg_rating * rating_weight;
        
        weighted_tips + weighted_rating
    }

    

    fn get_educator_reputation(
        env: &Env,
        educator: Address,
    ) -> Result<(i128, i128, i128, educator_contract::Educator), Error> {
        // Get educator contract address from storage
        let educator_contract = env.storage().instance().get(&EDUCATOR_CONTRACT).unwrap();
        let educator_client = EducatorClient::new(&env, &educator_contract);
        
        // Get educator data and verify they exist
        let educator_data = educator_client.get_educator(&educator)
            .ok_or(Error::InvalidEducator)?;
        
        // Get tips history
        let tips_history: Vec<TipRecord> = env.storage().instance().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        let mut total_tips = 0_i128;
        
        // Sum up all tips for this educator
        for tip in tips_history.iter() {
            if tip.to == educator {
                total_tips += tip.amount;
            }
        }
        
        // Get reviews and calculate rating
        let reviews = educator_client.get_educator_reviews(&educator);
        let mut ratings = Vec::new(&env);
        for review in reviews.iter() {
            ratings.push_back(review.rating);
        }
        let avg_rating = Self::calculate_average_rating(ratings);
        
        // Calculate reputation score
        let reputation_score = Self::combine_reputation_metrics(total_tips, avg_rating);
        
        Ok((total_tips, avg_rating, reputation_score, educator_data))
    }

    
    // Get tips sent by a user
    fn get_tips_sent(env: &Env, user: Address) -> Vec<TipRecord> {
        let tips_history: Vec<TipRecord> = env.storage().instance().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        let mut user_tips = Vec::new(&env);

        for tip in tips_history.iter() {
            if tip.from == user {
                user_tips.push_back(tip);
            }
        }

        env.events().publish(
            (TIPS_HISTORY, symbol_short!("tips_sent")),
            user
        );

        user_tips
    }

    // Get tips received by a user
    fn get_tips_received(env: &Env, user: Address) -> Vec<TipRecord> {
        let tips_history: Vec<TipRecord> = env.storage().instance().get(&TIPS_HISTORY).unwrap_or(Vec::new(&env));
        let mut user_tips = Vec::new(&env);

        for tip in tips_history.iter() {
            if tip.to == user {
                user_tips.push_back(tip);
            }
        }

        env.events().publish(
            (TIPS_HISTORY, symbol_short!("tips_recv")),
            user
        );

        user_tips
    }
}

#[cfg(test)]
mod test;
mod datatypes;
mod interfaces;
