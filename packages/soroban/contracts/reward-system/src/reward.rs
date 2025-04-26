use soroban_sdk::symbol_short;
use soroban_sdk::{contractimpl, Address, Env};

use crate::datatype::RewardEvent;
use crate::datatype::RewardType;
use crate::events::emit_reward_issued;
use crate::interface::RewardTrait;
use crate::{BalanceTrait, Error, RewardSystem, RewardSystemArgs, RewardSystemClient};

#[contractimpl]
impl RewardTrait for RewardSystem {
    fn distribute_rewards(
        env: Env,
        recipient: Address,
        reward_type: RewardType,
        amount: i128,
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        <RewardSystem as BalanceTrait>::update_balance(env.clone(), recipient.clone(), amount)?;

        // let event = RewardEvent {
        //     recipient: recipient.clone(),
        //     reward_type,
        //     amount,
        //     timestamp: env.ledger().timestamp(),
        // };

        // env.events().publish(
        //     ("reward_issued", recipient),
        //     (reward_type, amount, env.ledger().timestamp()),
        // );
        // emit_reward_issued(&env, &event)?;
        env.events().publish(
            (symbol_short!("rd_issued"), recipient),
            (reward_type, amount, env.ledger().timestamp()),
        );
        // Self::log_reward_event(env.clone(), recipient.clone(), reward_type, amount);
        
        Ok(())
    }

    fn log_reward_event(env: Env, recipient: Address, reward_type: RewardType, amount: i128) {
        env.events().publish(
            (symbol_short!("rd_issued"),),
            (reward_type, recipient, amount, env.ledger().timestamp()),
        );
    }
}
