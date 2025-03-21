use soroban_sdk::{
    contract, contractimpl, token, Address, Env, String, Vec, symbol_short,
};
use crate::datatype::VerificationLevel;

#[contract]
pub struct NFTImplementation;

#[contractimpl]
impl NFTImplementation {
    pub fn initialize_nft(env: Env, _admin: Address) -> Address {
        // Verificamos que no esté ya inicializado
        if env.storage().instance().has(&symbol_short!("token")) {
            panic!("already initialized");
        }

        let contract_address = env.current_contract_address();
        // No usamos token aquí, pero lo guardamos para uso futuro
        
        // Guardamos el token_id
        env.storage().instance().set(&symbol_short!("token"), &contract_address);
        contract_address
    }

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
        
        // Creamos un ID único usando un contador
        let counter: u32 = env.storage().instance().get(&symbol_short!("counter")).unwrap_or(0);
        let nft_id = String::from_str(&env, "NFT");
        env.storage().instance().set(&symbol_short!("counter"), &(counter + 1));
        
        // Transferimos el token
        token.transfer(&admin, &recipient, &1);
        
        // Guardamos los metadatos
        let metadata = (level, specialties);
        env.storage().persistent().set(&nft_id, &metadata);
        
        nft_id
    }

    pub fn burn_nft(env: Env, nft_id: String) {
        let token_id: Address = env.storage().instance()
            .get(&symbol_short!("token"))
            .unwrap_or_else(|| panic!("Token not initialized"));
            
        let token = token::Client::new(&env, &token_id);
        let burn_address = env.current_contract_address();
        
        burn_address.require_auth();
        
        // Transferimos a la dirección de quemado
        token.transfer(&burn_address, &burn_address, &1);
        
        // Eliminamos los metadatos
        env.storage().persistent().remove(&nft_id);
    }
} 