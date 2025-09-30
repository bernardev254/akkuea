use soroban_sdk::{Address, Env};

use crate::Error;

/// Validate contribution amount
/// Ensures the contribution is positive and non-zero
pub fn validate_contribution(contribution: i128) -> Result<(), Error> {
    if contribution <= 0 {
        return Err(Error::InvalidContribution);
    }
    
    if contribution == 0 {
        return Err(Error::ZeroContribution);
    }
    
    Ok(())
}

/// Verify that the user is authorized (must be a valid Stellar account)
/// In production, this should verify against account flags or additional criteria
pub fn verify_user_authorization(_env: &Env, user: &Address) -> Result<(), Error> {
    // Require authentication from the user
    user.require_auth();
    
    // Additional verification could be added here:
    // - Check if account is on a whitelist
    // - Verify account age
    // - Check account trustlines
    // - Validate account flags
    
    Ok(())
}

/// Get current timestamp from the Soroban ledger
pub fn get_current_timestamp(env: &Env) -> u64 {
    env.ledger().timestamp()
}

/// Convert XLM to Stroops (1 XLM = 10,000,000 Stroops)
pub fn xlm_to_stroops(xlm: i128) -> i128 {
    xlm * 10_000_000
}

/// Convert Stroops to XLM (1 XLM = 10,000,000 Stroops)
pub fn stroops_to_xlm(stroops: i128) -> i128 {
    stroops / 10_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_contribution_positive() {
        assert!(validate_contribution(100).is_ok());
        assert!(validate_contribution(1000000).is_ok());
    }

    #[test]
    fn test_validate_contribution_zero() {
        assert_eq!(validate_contribution(0), Err(Error::InvalidContribution));
    }

    #[test]
    fn test_validate_contribution_negative() {
        assert_eq!(validate_contribution(-100), Err(Error::InvalidContribution));
    }

    #[test]
    fn test_xlm_stroops_conversion() {
        assert_eq!(xlm_to_stroops(1), 10_000_000);
        assert_eq!(xlm_to_stroops(100), 1_000_000_000);
        assert_eq!(stroops_to_xlm(10_000_000), 1);
        assert_eq!(stroops_to_xlm(1_000_000_000), 100);
    }
}
