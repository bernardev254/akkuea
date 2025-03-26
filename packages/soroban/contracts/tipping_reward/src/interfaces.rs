use crate::datatypes::{Error, TipRecord};
use soroban_sdk::{Address, Env, Vec};
use crate::educator_contract::Educator;

pub trait TippingRewardsInterface {
    // Constructor to save the educator-verification-nft contract address on deployment
    fn __constructor(env: &Env, educator_contract: Address);

    // Send a tip to an educator
    fn send_tip(
        env: &Env,
        token: Address,
        from: Address,
        educator: Address,
        amount: i128,
    ) -> Result<(), Error>;

    // Get top educators by reputation
    fn get_top_educators(env: &Env, limit: u32) -> Vec<Educator>;

    fn calculate_average_rating(ratings: Vec<u32>) -> i128;

    fn combine_reputation_metrics(total_tips: i128, avg_rating: i128) -> i128;

    fn get_educator_reputation(
        env: &Env,
        educator: Address,
    ) -> Result<(i128, i128, i128, Educator), Error>;

    // Queries
    fn get_tips_sent(env: &Env, user: Address) -> Vec<TipRecord>;
    fn get_tips_received(env: &Env, user: Address) -> Vec<TipRecord>;
   
}
