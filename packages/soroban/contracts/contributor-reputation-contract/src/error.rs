use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotAuthorized = 1,      // Caller is not authorized
    UserNotFound = 2,       // User does not exist
    AlreadyVerified = 3,    // User is already verified
    NotVerified = 4,        // User is not verified
    InvalidInput = 5,       // Invalid input provided
    TokenNotFound = 6,      // Credential token not found
    ReputationNotFound = 7, // Reputation data not found
}
