use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Invalid contribution amount (must be positive)
    InvalidContribution = 1,

    /// Tier not found for the user
    TierNotFound = 2,

    /// Unauthorized access (only verified Stellar accounts)
    Unauthorized = 3,

    /// Tier downgrade not allowed
    DowngradeNotAllowed = 4,

    /// Storage error
    StorageError = 5,

    /// Invalid tier level
    InvalidTierLevel = 6,

    /// Zero contribution not allowed
    ZeroContribution = 7,

    /// User already has a tier assigned
    TierAlreadyExists = 8,

    /// Greeting not eligible for rewards
    NotEligible = 9,

    /// Reward already claimed for greeting
    RewardAlreadyClaimed = 10,

    /// Greeting not found
    GreetingNotFound = 11,

    /// Cross-contract call failed
    ExternalCallFailed = 12,
}
