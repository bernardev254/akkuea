use soroban_sdk::{contracttype, token, Address, Env, Symbol, symbol_short, Vec, log, String};
use crate::rental::{Rental, get_rental_by_rental_id, get_rental};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance,
}

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    rental_id: u64,        // Associated rental ID
    amount: i128,          // Payment amount in XLM (Stroops)
    payer: Address,        // Stellar address of the payer
    timestamp: u64,        // Payment timestamp
}

pub const TOKEN: Symbol = symbol_short!("token");
pub const EQUIPMENT_PRICE: Symbol = symbol_short!("eq_price");


pub fn set_token_address(env: &Env, token: Address) {
    env.storage().persistent().set(&TOKEN, &token);
}

pub fn set_equipment_price(env: &Env, equipment_id: u64, price_per_hour: i128) {
    let equip_rental = get_rental(&env, equipment_id).unwrap();
    let equip_duration = equip_rental.duration;
    let total_rental_price = (equipment_id * equip_duration) as i128;
    let key = (EQUIPMENT_PRICE, equipment_id);
    env.storage().persistent().set(&key, &total_rental_price);
}

pub fn get_equipment_price(env: &Env, equipment_id: u64) -> i128 {
    let key = (EQUIPMENT_PRICE, equipment_id);
    env.storage().persistent().get(&key).expect("NO PRICE FOR EQUIPMENT")
}

pub fn process_payment(env: &Env, rental_id: u64, payer: Address, amount: i128) -> bool {
    payer.require_auth();

    let token = env.storage().persistent().get(&TOKEN).expect("TOKEN NOT YET INITIALIZED");
    let token_client = token::Client::new(&env, &token);
    let timestamp = env.ledger().timestamp();

    let rental_data: Rental = get_rental_by_rental_id(&env, rental_id).expect("REASON");
    let rental_duration = rental_data.duration;
    let rental_equipment_id = rental_data.equipment_id;
    let rent_per_hour = get_equipment_price(&env, rental_equipment_id);
    let total_rental_amount = rental_duration as i128 * rent_per_hour;

    // let xlm_asset = env.register_stellar_asset_contract(Address::from_str(&env, "native"));

    log!(&env, "total_rental_amount: {:?}", total_rental_amount);

    if token_client.balance(&payer) < total_rental_amount ||  amount < total_rental_amount{
        panic!("Not enough funds");
    }
    
    token_client.transfer(&payer, &env.current_contract_address(), &amount);

    env.storage().instance().set(
        &DataKey::Balance,
             &Payment {
                rental_id, 
                amount,
                payer,
                timestamp,  
            },
        );
    true
}