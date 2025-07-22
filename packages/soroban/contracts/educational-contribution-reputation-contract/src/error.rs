use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotAuthorized = 1,           // Caller is not authorized
    UserNotFound = 2,            // User does not exist
    AlreadyVerified = 3,         // User is already verified
    NotVerified = 4,             // User is not verified
    InvalidInput = 5,            // Invalid input provided
    TokenNotFound = 6,           // Credential token not found
    ReputationNotFound = 7,      // Reputation data not found
    DisputeNotFound = 8,         // Dispute not found
    DisputeAlreadyExists = 9,    // Dispute already exists for this reputation
    DisputeAlreadyResolved = 10, // Dispute has already been resolved
    InsufficientEvidence = 11,   // Insufficient evidence for dispute
    RecoveryNotAllowed = 12,     // Recovery not allowed for this user
    ProbationActive = 13,        // User is currently on probation
    InvalidTimeRange = 14,       // Invalid time range for analytics
    InsufficientData = 15,       // Insufficient data for analytics
}
