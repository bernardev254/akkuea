use soroban_sdk::{
    contract, contractimpl, token, Address, Env, String, Vec, symbol_short,
};
use crate::datatype::VerificationLevel;

/// NFT implementation for educator verification credentials
#[contract]
pub struct NFTImplementation;

#[contractimpl]
impl NFTImplementation {
    /// Initialize the NFT contract and return its address
    pub fn initialize_nft(env: Env, _admin: Address) -> Address {
        // Check that it's not already initialized
        if env.storage().instance().has(&symbol_short!("token")) {
            panic!("already initialized");
        }

        let contract_address = env.current_contract_address();
        // We don't use token here, but save it for future use
        
        // Store the token_id
        env.storage().instance().set(&symbol_short!("token"), &contract_address);
        contract_address
    }

    /// Mint a new NFT for an educator with a specific verification level
    pub fn mint_nft(
        env: Env, 
        admin: Address, 
        recipient: Address, 
        level: VerificationLevel,
        specialties: Vec<String>
    ) -> String {
        admin.require_auth();

        let token_id = env.storage().instance()
            .get(&symbol_short!("token"))
            .unwrap_or_else(|| NFTImplementation::initialize_nft(env.clone(), admin.clone()));
            
        let token = token::Client::new(&env, &token_id);
        
        // Create a unique ID using a counter
        let counter: u32 = env.storage().instance().get(&symbol_short!("counter")).unwrap_or(0);
        let nft_id = String::from_str(&env, "NFT");
        env.storage().instance().set(&symbol_short!("counter"), &(counter + 1));
        
        // Transfer the token
        token.transfer(&admin, &recipient, &1);
        
        // Store the metadata
        let metadata = (level, specialties);
        env.storage().persistent().set(&nft_id, &metadata);
        
        nft_id
    }

    /// Burn an NFT by its ID, removing it from circulation
    pub fn burn_nft(env: Env, nft_id: String) {
        let token_id: Address = env.storage().instance()
            .get(&symbol_short!("token"))
            .unwrap_or_else(|| panic!("Token not initialized"));
            
        let token = token::Client::new(&env, &token_id);
        let burn_address = env.current_contract_address();
        
        burn_address.require_auth();
        
        // Transfer to the burn address
        token.transfer(&burn_address, &burn_address, &1);
        
        // Remove the metadata
        env.storage().persistent().remove(&nft_id);
    }
} 