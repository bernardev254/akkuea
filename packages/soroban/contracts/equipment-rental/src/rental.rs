use soroban_sdk::{contracttype, Address, Env, Symbol, symbol_short, Vec, log, vec};
use crate::utils::{validate_duration, generate_rental_id};
use crate::payment::{get_payment_by_rental_id};

pub const RENTAL_KEY: Symbol = symbol_short!("rentals");
pub const MAX_DURATION: Symbol = symbol_short!("max_dur");
pub const RENTAL_CREATED: Symbol = symbol_short!("r_created");
pub const RENTAL_FAILED: Symbol = symbol_short!("r_failed");

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum RentalStatus {
    Pending,    // Rental created, awaiting payment
    Active,     // Payment processed, rental active
    Completed,  // Rental duration ended
    Cancelled,  // Rental cancelled before start
}

#[contracttype]
#[derive(Clone)]
pub struct Rental {
    pub rental_id: u64,
    pub equipment_id: u64,
    pub renter: Address,
    pub start_time: u64,
    pub duration: u64,
    pub status: RentalStatus,
}


pub fn create_rental(env: &Env, renter: Address, equipment_id: u64, duration: u64) {
    renter.require_auth();

    validate_duration(&env, duration);

    if check_availability(&env, equipment_id) == false {
        env.events().publish((RENTAL_FAILED, equipment_id), "equipment_unavailable");
        panic!("Equipment not available");
    }

    let rental_id = generate_rental_id(&env);
    
    let rental = Rental {
        rental_id,
        equipment_id,
        renter: renter.clone(),
        start_time: env.ledger().timestamp(),
        duration,
        status: RentalStatus::Pending,
    };

    save_rental(&env, &rental);
    env.events().publish((RENTAL_CREATED, rental_id), (equipment_id, renter, duration));
}


pub fn save_rental(env: &Env, rental: &Rental) {
    let mut rentals: Vec<Rental> = env.storage().persistent().get(&RENTAL_KEY).unwrap_or(Vec::new(&env));
    rentals.push_back(rental.clone());
    env.storage().persistent().set(&RENTAL_KEY, &rentals);
}

pub fn get_rentals_by_equipment_id(env: &Env, equipment_id: u64) -> Vec<Rental> {
    let rentals: Vec<Rental> = env.storage().persistent().get(&RENTAL_KEY).unwrap_or(vec![env]);
    let mut result = vec![env];
    for rental in rentals.iter() {
        if rental.equipment_id == equipment_id {
            result.push_back(rental);
        }
    }
    result
}

pub fn get_rental_by_rental_id(env: &Env, rental_id: u64) -> Option<Rental> {
    let rentals: Option<Vec<Rental>> = env.storage().persistent().get(&RENTAL_KEY).unwrap();
    let rental = rentals
        .and_then(|ps| ps.iter().find(|p| p.rental_id == rental_id))
        .unwrap_or_else(|| panic!("Payment not found"));
    Some(rental)
}

pub fn update_rental_status(env: &Env, rental_id: u64, rental_status: RentalStatus) -> bool {
    log!(&env, "UPDATING RENTAL STATE");

    let mut rentals: Vec<Rental> = env.storage().persistent().get(&RENTAL_KEY).unwrap_or(vec![env]);
    if let Some(index) = rentals.iter().position(|r| r.rental_id == rental_id) {
        let mut rental = rentals.get_unchecked((index as usize).try_into().unwrap()).clone();
        rental.status = rental_status;
        rentals.set(index as u32, rental);
        env.storage().persistent().set(&RENTAL_KEY, &rentals);
        true
    } else {
        false
    }
}

pub fn check_availability(env: &Env, equipment_id: u64) -> bool {
    let rentals = get_rentals_by_equipment_id(env, equipment_id);
    let timestamp = env.ledger().timestamp();

    for rental in rentals {
        let payment_by_rental = get_payment_by_rental_id(&env, rental.rental_id);

        match rental.status {
            RentalStatus::Pending => {
                log!(&env, "PENDING RENTAL DETECTED");
                if payment_by_rental.is_some() {
                    log!(&env, "PENDING RENTAL WITH PAYMENT");
                    return false; // Pending with payment, not available
                }
                continue; // Pending without payment, considered available
            }
            RentalStatus::Active => {
                if let Some(payment) = payment_by_rental {
                    let rental_end_time = payment.timestamp + rental.duration;
                    if rental_end_time < timestamp {
                        log!(&env, "EXPIRED ACTIVE RENTAL");
                        update_rental_status(&env, rental.rental_id, RentalStatus::Completed);
                        continue; // Expired, considered available
                    }
                    log!(&env, "ACTIVE RENTAL NOT EXPIRED");
                    return false; // Active, not expired, not available
                }
            }
            RentalStatus::Completed | RentalStatus::Cancelled => {
                continue; // Completed or cancelled, considered available
            }
        }
    }
    true // No blocking rentals, equipment available
}
