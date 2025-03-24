use soroban_sdk::{Address, String, Vec, contracttype, Symbol, contracterror};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InvalidEducator = 2,
    NotAuthorized = 3,
    EducatorNotVerified = 4,
}

// Storage keys
const EDUCATORS: Symbol = Symbol::short("EDUC");
const TIPS_HISTORY: Symbol = Symbol::short("TIPS");

#[derive(Clone)]
#[contracttype]
pub struct EducatorTipInfo {
    address: Address,
    total_tips_received: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct TipRecord {
    from: Address,
    to: Address,
    amount: i128,
    timestamp: u64,
}
