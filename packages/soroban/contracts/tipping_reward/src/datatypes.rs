use soroban_sdk::{Address, contracttype, Symbol, contracterror, symbol_short};

// Storage keys
pub const TIPS_HISTORY: Symbol = symbol_short!("TIPS");

#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InvalidEducator = 2,
    NotAuthorized = 3,
    EducatorNotVerified = 4,
    InvalidRating = 5,
    ReviewFailed = 6,
}


// #[derive(Clone)]
// #[contracttype]
// pub struct EducatorTipInfo {
//     pub address: Address,
//     pub total_tips_received: i128,
// }

#[derive(Clone)]
#[contracttype]
pub struct TipRecord {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub timestamp: u64,
}
