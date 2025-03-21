use soroban_sdk::{Address, Env, String, Vec};
use crate::datatype::{Educator, VerificationLevel, Review};

pub trait EducatorVerificationInterface {
    // Funciones de administración
    fn initialize(env: Env, admin: Address);
    fn add_reviewer(env: Env, admin: Address, reviewer: Address);
    fn remove_reviewer(env: Env, admin: Address, reviewer: Address);
    
    // Funciones de educador
    fn register_educator(env: Env, educator_address: Address, name: String, credential_hashes: Vec<String>, specialty_areas: Vec<String>) -> Address;
    fn update_educator_profile(env: Env, educator_address: Address, name: Option<String>, specialty_areas: Option<Vec<String>>) -> bool;
    fn add_credentials(env: Env, educator_address: Address, new_credentials: Vec<String>) -> bool;
    
    // Funciones de verificación
    fn verify_educator(env: Env, reviewer: Address, educator_address: Address, verification_level: VerificationLevel);
    fn revoke_verification(env: Env, admin: Address, educator_address: Address, reason: String);
    
    // Nuevas funciones para gestión de credenciales e instituciones
    fn add_verified_credential(env: Env, reviewer: Address, credential: String);
    fn add_authorized_institution(env: Env, admin: Address, institution_id: String);
    
    // Funciones de consulta
    fn get_educator(env: Env, educator_address: Address) -> Option<Educator>;
    fn get_verified_educators(env: Env) -> Vec<Address>;
    fn get_educators_by_specialty(env: Env, specialty: String) -> Vec<Address>;
    fn get_educator_reviews(env: Env, educator_address: Address) -> Vec<Review>;
    
    // Funciones de revisión
    fn submit_review(env: Env, reviewer: Address, educator_address: Address, rating: u32);
} 