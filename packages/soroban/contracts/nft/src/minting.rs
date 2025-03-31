use soroban_sdk::{contractimpl, Address, Env, String, Vec, Map, BytesN};
use crate::{NFTMetadata, NFTDetail, PurchaseMetadata, COUNTER_KEY, TRANSACTION_MAPPING};

#[contractimpl]
impl super::AkkueaPurchaseNFT {
    /// Mint a new NFT representing proof of purchase
    pub fn mint_proof_of_purchase(
        env: Env,
        buyer: Address,
        seller: Address,
        transaction_id: BytesN<32>,
        purchase_id: String,
        amount: i128,
        currency: String,
        product_id: String,
        product_name: String,
        name: String,
        description: String,
        attributes: Vec<String>,
        additional_attributes: Map<String, String>,
    ) -> u32 {
        // Ensure transaction authenticity
        seller.require_auth();
        
        // Validate transaction hasn't been used before
        let mut txn_map: Map<BytesN<32>, u32> = env.storage().instance().get(&TRANSACTION_MAPPING).unwrap();
        if txn_map.contains_key(&transaction_id) {
            panic!("Transaction already has an associated NFT");
        }
        
        // Get current timestamp from ledger
        let timestamp = env.ledger().timestamp();
        
        // Create purchase metadata
        let purchase_data = PurchaseMetadata {
            purchase_id,
            timestamp,
            amount,
            currency,
            product_id,
            product_name,
            additional_attributes,
        };
        
        // Create NFT metadata
        let metadata = NFTMetadata {
            name,
            description,
            purchase_data,
            attributes,
        };
        
        // Create NFT detail
        let nft = NFTDetail {
            owner: buyer.clone(),
            seller: seller.clone(),
            metadata,
            transaction_id: transaction_id.clone(),
        };
        
        // Increment token counter
        let mut current_id: u32 = env.storage().instance().get(&COUNTER_KEY).unwrap();
        current_id += 1;
        env.storage().instance().set(&COUNTER_KEY, &current_id);
        
        // Store NFT
        env.storage().persistent().set(&current_id, &nft);
        
        // Map transaction to NFT for future reference
        txn_map.set(transaction_id, current_id);
        env.storage().instance().set(&TRANSACTION_MAPPING, &txn_map);
        
        // Log the minting event
        env.events().publish(("mint", "proof_of_purchase"), 
            (buyer, seller, purchase_id, current_id));
            
        // Return the token ID
        current_id
    }
    
    /// Admin function to batch mint NFTs for historical purchases
    pub fn admin_batch_mint(
        env: Env,
        admin: Address,
        batch_data: Vec<(
            Address, Address, BytesN<32>, String, i128, 
            String, String, String, String, String, 
            Vec<String>, Map<String, String>
        )>,
    ) -> Vec<u32> {
        // Check admin privileges
        Self::check_admin(&env, &admin);
        admin.require_auth();
        
        let mut minted_ids: Vec<u32> = Vec::new(&env);
        
        // Process each batch entry
        for data in batch_data.iter() {
            let (
                buyer, seller, txn_id, purchase_id, amount,
                currency, product_id, product_name, name, description,
                attributes, additional_attributes
            ) = data.clone();
            
            // Skip if transaction already has an NFT
            let txn_map: Map<BytesN<32>, u32> = env.storage().instance().get(&TRANSACTION_MAPPING).unwrap();
            if txn_map.contains_key(&txn_id) {
                continue;
            }
            
            // Mint the NFT
            let token_id = Self::mint_proof_of_purchase(
                env.clone(), buyer, seller, txn_id, purchase_id,
                amount, currency, product_id, product_name, 
                name, description, attributes, additional_attributes
            );
            
            minted_ids.push_back(token_id);
        }
        
        minted_ids
    }
}
