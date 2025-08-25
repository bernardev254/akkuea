use soroban_sdk::{contracttype, token, Address, Env, Symbol, symbol_short, Vec, log};
use crate::rental::{Rental, update_rental_status, get_rental_by_rental_id, check_availability, RENTAL_FAILED, RentalStatus};

pub const PAYMENT_KEY: Symbol = symbol_short!("payments");
pub const PAYMENT_SUCCESS: Symbol = symbol_short!("p_success");


#[derive(Clone)]
#[contracttype]
pub enum DataKey { 
    Payments(Address), // tracking payments by payer
    RentalPayments(u64) // tracking all payments for a specific rental ID.
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Payment {
    pub rental_id: u64,        // Associated rental ID
    pub amount: i128,          // Payment amount in XLM (Stroops)
    pub payer: Address,        // Stellar address of the payer
    pub timestamp: u64,        // Payment timestamp
}

pub const TOKEN: Symbol = symbol_short!("token");
pub const EQUIPMENT_PRICE: Symbol = symbol_short!("eq_price");

pub fn set_token_address(env: &Env, token: Address) {
    env.storage().persistent().set(&TOKEN, &token);
}

pub fn set_equipment_price(env: &Env, equipment_id: u64, price_per_hour: i128) {
    let key = (EQUIPMENT_PRICE, equipment_id);
    env.storage().persistent().set(&key, &price_per_hour);
}

pub fn get_equipment_price(env: &Env, equipment_id: u64) -> i128 {
    let key = (EQUIPMENT_PRICE, equipment_id);
    env.storage().persistent().get(&key).expect("NO PRICE FOR EQUIPMENT")
}

pub fn get_payments(env: &Env, ) -> Vec<Payment> {
    let payments: Vec<Payment> = env.storage().persistent().get(&PAYMENT_KEY).unwrap_or(Vec::new(&env));
    payments
}

pub fn get_payment_by_rental_id(env: &Env, rental_id: u64) -> Option<Payment> {
    let payments: Option<Vec<Payment>> = env.storage().persistent().get(&symbol_short!("payments"));
    let payment = payments.and_then(|ps| ps.iter().find(|p| p.rental_id == rental_id));
    payment
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

    if check_availability(&env, rental_equipment_id) == false {
        env.events().publish((RENTAL_FAILED, rental_equipment_id), "equipment_unavailable");
        panic!("Equipment Already Paid For");
    }

    if token_client.balance(&payer) < total_rental_amount ||  amount < total_rental_amount{
        panic!("Insufficient balance/amount");
    }
    
    token_client.transfer(&payer, &env.current_contract_address(), &amount);
    let payment = Payment {
                rental_id, 
                amount,
                payer: payer.clone(),
                timestamp,  
            };
    
    let mut payments: Vec<Payment> = env.storage().persistent().get(&PAYMENT_KEY).unwrap_or(Vec::new(env));
    payments.push_back(payment.clone());
    env.storage().persistent().set(&PAYMENT_KEY, &payments);

    let res = update_rental_status(&env, rental_data.rental_id, RentalStatus::Active);
    log!(&env, "IS RENTAL STATE UPDATED: {}", res);

    env.events().publish((PAYMENT_SUCCESS, rental_id), (payer, amount));
    true
}
