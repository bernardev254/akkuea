use soroban_sdk::{Address, Env, Vec};
use crate::datatypes::{Error, EducatorTipInfo, TipRecord};
use crate::educator_contract::Review;

pub trait TippingRewardsInterface {
    // Send a tip to an educator
    fn send_tip(
        env: &Env,
        token_id: Address,
        from: Address,
        educator: Address,
        amount: i128,
    ) -> Result<(), Error>;

    // Get top educators by reputation
    fn get_top_educators(
        env: &Env, 
        educator_contract: Address, 
        limit: u32
    ) -> Vec<Address>;

    // Get educator info combining local tip data and educator contract data
    fn get_educator_info(
        env: &Env, 
        educator_contract: Address, 
        educator: Address
    ) -> Option<(EducatorTipInfo, Vec<Review>)>;

    // Get tips history for an educator
    fn get_educator_tips(
        env: &Env, 
        educator: Address
    ) -> Vec<TipRecord>;
}