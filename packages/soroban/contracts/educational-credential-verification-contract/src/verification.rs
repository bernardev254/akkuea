use soroban_sdk::{Address, Env, Vec, Map, String, symbol_short};
use crate::datatype::{VerificationLevel, Review};
use crate::nft::NFTImplementation;

pub struct VerificationSystem;

impl VerificationSystem {
    pub fn has_administrator(env: &Env) -> bool {
        env.storage().instance().has(&symbol_short!("admin"))
    }

    pub fn verify_admin(env: &Env, admin: &Address) {
        let stored_admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        if admin != &stored_admin {
            panic!("not authorized");
        }
    }

    pub fn is_reviewer(env: &Env, reviewer: &Address) -> bool {
        let reviewers: Vec<Address> = env.storage().persistent()
            .get(&symbol_short!("REVIEWERS"))
            .unwrap_or_else(|| Vec::new(env));
        reviewers.contains(reviewer)
    }

    pub fn add_reviewer(env: &Env, reviewer: &Address) {
        let mut reviewers: Vec<Address> = env.storage().persistent()
            .get(&symbol_short!("REVIEWERS"))
            .unwrap_or_else(|| Vec::new(env));
        if !reviewers.contains(reviewer) {
            reviewers.push_back(reviewer.clone());
            env.storage().persistent().set(&symbol_short!("REVIEWERS"), &reviewers);
        }
    }

    pub fn remove_reviewer(env: &Env, reviewer: &Address) {
        let reviewers: Vec<Address> = env.storage().persistent()
            .get(&symbol_short!("REVIEWERS"))
            .unwrap_or_else(|| Vec::new(env));
        
        let mut new_reviewers = Vec::new(env);
        for r in reviewers.iter() {
            if &r != reviewer {
                new_reviewers.push_back(r.clone());
            }
        }
        
        env.storage().persistent().set(&symbol_short!("REVIEWERS"), &new_reviewers);
    }

    pub fn verify_credentials(env: &Env, credentials: &Vec<String>, reviewer: &Address) -> bool {
        if !Self::is_reviewer(env, reviewer) {
            return false;
        }

        let verified_credentials: Map<String, bool> = env.storage().persistent()
            .get(&symbol_short!("vcreds"))
            .unwrap_or_else(|| Map::new(env));

        for credential in credentials.iter() {
            if let Some(is_verified) = verified_credentials.get(credential.clone()) {
                if !is_verified {
                    return false;
                }
                continue;
            }

            if !Self::is_valid_hash_format(env, &credential) {
                return false;
            }

            if !Self::verify_digital_signature(env, &credential, reviewer) {
                return false;
            }

            if !Self::verify_institution(env, &credential) {
                return false;
            }
        }

        true
    }

    fn is_valid_hash_format(_env: &Env, hash: &String) -> bool {
        hash.len() == 64
    }

    fn verify_digital_signature(env: &Env, credential: &String, reviewer: &Address) -> bool {
        let signatures: Map<String, Vec<Address>> = env.storage().persistent()
            .get(&symbol_short!("sigs"))
            .unwrap_or_else(|| Map::new(env));

        if let Some(signers) = signatures.get(credential.clone()) {
            signers.contains(reviewer)
        } else {
            false
        }
    }

    fn verify_institution(env: &Env, credential: &String) -> bool {
        let authorized_institutions: Vec<String> = env.storage().persistent()
            .get(&symbol_short!("AUTH_INST"))
            .unwrap_or_else(|| Vec::new(env));

        authorized_institutions.contains(credential)
    }

    pub fn add_verified_credential(env: &Env, credential: String, reviewer: &Address) {
        if !Self::is_reviewer(env, reviewer) {
            panic!("not authorized reviewer");
        }

        let mut verified_credentials: Map<String, bool> = env.storage().persistent()
            .get(&symbol_short!("vcreds"))
            .unwrap_or_else(|| Map::new(env));

        verified_credentials.set(credential.clone(), true);
        env.storage().persistent().set(&symbol_short!("vcreds"), &verified_credentials);

        let mut signatures: Map<String, Vec<Address>> = env.storage().persistent()
            .get(&symbol_short!("sigs"))
            .unwrap_or_else(|| Map::new(env));

        let mut signers = signatures.get(credential.clone())
            .unwrap_or_else(|| Vec::new(env));
        
        if !signers.contains(reviewer) {
            signers.push_back(reviewer.clone());
            signatures.set(credential, signers);
            env.storage().persistent().set(&symbol_short!("sigs"), &signatures);
        }
    }

    pub fn add_authorized_institution(env: &Env, admin: &Address, institution_id: String) {
        Self::verify_admin(env, admin);

        let mut authorized_institutions: Vec<String> = env.storage().persistent()
            .get(&symbol_short!("AUTH_INST"))
            .unwrap_or_else(|| Vec::new(env));

        if !authorized_institutions.contains(&institution_id) {
            authorized_institutions.push_back(institution_id);
            env.storage().persistent().set(&symbol_short!("AUTH_INST"), &authorized_institutions);
        }
    }

    pub fn mint_verification_nft(
        env: &Env, 
        recipient: &Address, 
        level: &VerificationLevel,
        specialties: &Vec<String>
    ) -> String {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        NFTImplementation::mint_nft(
            env.clone(), 
            admin, 
            recipient.clone(), 
            level.clone(), 
            specialties.clone()
        )
    }

    pub fn store_review(env: &Env, review: Review) {
        let key = symbol_short!("revs");
        let mut reviews: Map<Address, Vec<Review>> = env.storage().persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(env));
        
        let mut educator_reviews = reviews.get(review.educator.clone())
            .unwrap_or_else(|| Vec::new(env));
        
        educator_reviews.push_back(review.clone());
        reviews.set(review.educator.clone(), educator_reviews);
        
        env.storage().persistent().set(&key, &reviews);
    }

    pub fn calculate_verification_level(env: &Env, educator: &Address) -> VerificationLevel {
        let reviews: Map<Address, Vec<Review>> = env.storage().persistent()
            .get(&symbol_short!("revs"))
            .unwrap_or_else(|| Map::new(env));
        
        if let Some(educator_reviews) = reviews.get(educator.clone()) {
            let avg_rating: u32 = educator_reviews.iter()
                .map(|r| r.rating)
                .sum::<u32>() / educator_reviews.len() as u32;
            
            match avg_rating {
                0..=3 => VerificationLevel::Basic,
                4..=7 => VerificationLevel::Advanced,
                _ => VerificationLevel::Expert,
            }
        } else {
            VerificationLevel::Basic
        }
    }
} 