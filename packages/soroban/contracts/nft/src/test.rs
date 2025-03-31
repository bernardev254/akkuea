#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, String, Map, Vec, IntoVal,
};
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};

use crate::{AkkueaPurchaseNFT, AkkueaPurchaseNFTClient, NFTDetail, NFTMetadata, PurchaseMetadata};
use crate::minting::{PurchaseNFTData, ProductInfo, NFTMetaInput};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Verify admin was set correctly by checking total NFTs count
    // This is indirect verification since we don't expose admin getter
    assert_eq!(client.get_total_nfts(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_double_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize once
    client.initialize(&admin);
    
    // Try to initialize again (should panic)
    client.initialize(&admin);
}

#[test]
fn test_mint_nft() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create additional attributes
    let mut attributes = Map::new(&env);
    attributes.set(String::from_str(&env, "color"), String::from_str(&env, "blue"));
    
    // Mock auth for seller
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &purchase_data,
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Create purchase data
    let product_info = ProductInfo {
        product_id: String::from_str(&env, "PROD-001"),
        product_name: String::from_str(&env, "Premium Widget"),
    };
    
    let nft_meta = NFTMetaInput {
        name: String::from_str(&env, "Purchase NFT"),
        description: String::from_str(&env, "Proof of purchase for Widget"),
        attributes: Vec::<String>::new(&env),
        additional_attributes: attributes,
    };
    
    let purchase_data = PurchaseNFTData {
        buyer,
        seller,
        transaction_id: txn_id,
        purchase_id: String::from_str(&env, "PUR-12345"),
        amount: 100_000_000i128,
        currency: String::from_str(&env, "XLM"),
        product_info,
        nft_metadata: nft_meta,
    };
    
    // Mint the NFT
    let token_id = client.mint_proof_of_purchase(&purchase_data);
    
    // Verify the NFT was minted with ID 1
    assert_eq!(token_id, 1);
    
    // Verify NFT data
    let nft_info = client.get_nft_info(&token_id);
    assert_eq!(nft_info.owner, buyer);
    assert_eq!(nft_info.seller, seller);
    assert_eq!(nft_info.transaction_id, txn_id);
    assert_eq!(nft_info.metadata.name, String::from_str(&env, "Purchase NFT"));
    
    // Verify transaction mapping
    assert!(client.has_transaction_nft(&txn_id));
    assert_eq!(client.get_nft_by_transaction(&txn_id), Some(token_id));
    
    // Verify total NFT count
    assert_eq!(client.get_total_nfts(), 1);
}

#[test]
#[should_panic(expected = "Transaction already has an associated NFT")]
fn test_double_mint_same_transaction() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID (same for both mints)
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller
    seller.mock_auths(&[
        // First mint auth
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "simple_mint",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, 
                &String::from_str(&env, "Premium Widget")
            ).into_val(&env),
            auth_amount: None,
        }),
        // Second mint auth (should fail)
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // First mint
    client.mint_proof_of_purchase(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, &String::from_str(&env, "XLM"),
        &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
        &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
        &Vec::<String>::new(&env), &attributes
    );
    
    // Second mint with same transaction (should panic)
    client.mint_proof_of_purchase(&purchase_data);
}

#[test]
fn test_transfer_nft() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let new_owner = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Use the simple mint function for testing transfers
    let token_id = client.simple_mint(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, 
        &String::from_str(&env, "Premium Widget")
    );
    
    // Mock auth for buyer to transfer
    buyer.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "transfer_nft",
            args: (
                &buyer, &new_owner, &token_id
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Transfer the NFT
    client.transfer_nft(&buyer, &new_owner, &token_id);
    
    // Verify new ownership
    let nft_info = client.get_nft_info(&token_id);
    assert_eq!(nft_info.owner, new_owner);
    
    // Verify ownership check
    assert!(client.verify_ownership(&token_id, &new_owner));
    assert!(!client.verify_ownership(&token_id, &buyer));
}

#[test]
#[should_panic(expected = "Unauthorized: you are not the owner of this NFT")]
fn test_unauthorized_transfer() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let attacker = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Use the simple mint function for testing burns
    let token_id = client.simple_mint(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, 
        &String::from_str(&env, "Premium Widget")
    );
    
    // Mock auth for attacker trying to transfer
    attacker.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "transfer_nft",
            args: (
                &attacker, &attacker, &token_id
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Attempt unauthorized transfer (should panic)
    client.transfer_nft(&attacker, &attacker, &token_id);
}

#[test]
fn test_burn_nft() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Use the simple mint function for testing metadata
    let token_id = client.simple_mint(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, 
        &String::from_str(&env, "Premium Widget")
    );
    
    // Mock auth for buyer to burn
    buyer.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "burn_nft",
            args: (
                &buyer, &token_id
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Burn the NFT
    client.burn_nft(&buyer, &token_id);
    
    // Verify NFT is gone through validation
    assert!(!client.verify_nft(&token_id));
}

#[test]
fn test_update_metadata() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Use the simple mint function for testing validation
    let token_id = client.simple_mint(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, 
        &String::from_str(&env, "Premium Widget")
    );
    
    // New attributes for update
    let new_attributes = Vec::from_array(
        &env,
        [
            String::from_str(&env, "limited"),
            String::from_str(&env, "collector"),
        ],
    );
    
    // Mock auth for admin to update metadata
    admin.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "update_metadata",
            args: (
                &admin, &token_id,
                &String::from_str(&env, "Updated NFT Name"),
                &String::from_str(&env, "Updated description"),
                &new_attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Update metadata
    client.update_metadata(
        &admin, &token_id,
        &String::from_str(&env, "Updated NFT Name"),
        &String::from_str(&env, "Updated description"),
        &new_attributes
    );
    
    // Verify metadata was updated
    let metadata = client.get_metadata(&token_id);
    assert_eq!(metadata.name, String::from_str(&env, "Updated NFT Name"));
    assert_eq!(metadata.description, String::from_str(&env, "Updated description"));
    assert_eq!(metadata.attributes, new_attributes);
    
    // Verify purchase data remained intact
    let purchase_data = client.get_purchase_data(&token_id);
    assert_eq!(purchase_data.purchase_id, String::from_str(&env, "PUR-12345"));
    assert_eq!(purchase_data.amount, 100_000_000i128);
}

#[test]
fn test_add_attribute() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Mint the NFT
    let token_id = client.mint_proof_of_purchase(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, &String::from_str(&env, "XLM"),
        &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
        &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
        &Vec::<String>::new(&env), &attributes
    );
    
    // Mock auth for admin to add attribute
    admin.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "add_attribute",
            args: (
                &admin, &token_id,
                &String::from_str(&env, "warranty"),
                &String::from_str(&env, "1 year")
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Add attribute
    client.add_attribute(
        &admin, &token_id,
        &String::from_str(&env, "warranty"),
        &String::from_str(&env, "1 year")
    );
    
    // Verify attribute was added
    let purchase_data = client.get_purchase_data(&token_id);
    let added_attr = purchase_data.additional_attributes.get(
        &String::from_str(&env, "warranty")
    ).unwrap();
    
    assert_eq!(added_attr, String::from_str(&env, "1 year"));
}

#[test]
fn test_validation() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction ID
    let txn_id = BytesN::<32>::random(&env);
    
    // Create attributes
    let attributes = Map::new(&env);
    
    // Mock auth for seller to mint
    seller.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_proof_of_purchase",
            args: (
                &buyer, &seller, &txn_id, 
                &String::from_str(&env, "PUR-12345"),
                &100_000_000i128, &String::from_str(&env, "XLM"),
                &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
                &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
                &Vec::<String>::new(&env), &attributes
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Mint the NFT
    let token_id = client.mint_proof_of_purchase(
        &buyer, &seller, &txn_id,
        &String::from_str(&env, "PUR-12345"),
        &100_000_000i128, &String::from_str(&env, "XLM"),
        &String::from_str(&env, "PROD-001"), &String::from_str(&env, "Premium Widget"),
        &String::from_str(&env, "Purchase NFT"), &String::from_str(&env, "Proof of purchase for Widget"),
        &Vec::<String>::new(&env), &attributes
    );
    
    // Run validation tests
    
    // Verify NFT exists and is valid
    assert!(client.verify_nft(&token_id));
    
    // Verify non-existent NFT returns false
    assert!(!client.verify_nft(&99));
    
    // Verify purchase information
    assert!(client.verify_purchase(&token_id, &String::from_str(&env, "PUR-12345")));
    assert!(!client.verify_purchase(&token_id, &String::from_str(&env, "WRONG-ID")));
    
    // Verify ownership
    assert!(client.verify_ownership(&token_id, &buyer));
    assert!(!client.verify_ownership(&token_id, &seller));
    
    // Get validation report
    let report = client.validation_report(&token_id);
    assert!(report.get(&String::from_str(&env, "exists")).unwrap());
    assert!(report.get(&String::from_str(&env, "transaction_consistency")).unwrap());
}

#[test]
fn test_admin_batch_mint() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let buyer1 = Address::generate(&env);
    let buyer2 = Address::generate(&env);
    let seller = Address::generate(&env);
    
    let contract_id = env.register_contract(None, AkkueaPurchaseNFT {});
    let client = AkkueaPurchaseNFTClient::new(&env, &contract_id);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create transaction IDs
    let txn_id1 = BytesN::<32>::random(&env);
    let txn_id2 = BytesN::<32>::random(&env);
    
    // Create empty attributes maps
    let attributes1 = Map::new(&env);
    let attributes2 = Map::new(&env);
    
    // Create batch data with simplified parameters
    let batch_data = Vec::from_array(
        &env,
        [
            (
                buyer1.clone(), seller.clone(), txn_id1.clone(), 
                String::from_str(&env, "PUR-0001"),
                100_000_000i128, String::from_str(&env, "Widget A")
            ),
            (
                buyer2.clone(), seller.clone(), txn_id2.clone(), 
                String::from_str(&env, "PUR-0002"),
                200_000_000i128, String::from_str(&env, "Widget B")
            ),
        ],
    );
    
    // Mock auth for admin to batch mint
    admin.mock_auths(&[
        (&MockAuthInvoke {
            contract: &contract_id,
            fn_name: "admin_batch_mint",
            args: (
                &admin, &batch_data
            ).into_val(&env),
            auth_amount: None,
        }),
    ]);
    
    // Batch mint
    let token_ids = client.admin_batch_mint(&admin, &batch_data);
    
    // Verify two NFTs were minted
    assert_eq!(token_ids.len(), 2);
    assert_eq!(token_ids.get(0).unwrap(), 1);
    assert_eq!(token_ids.get(1).unwrap(), 2);
    
    // Verify each NFT's data
    let nft1 = client.get_nft_info(&1);
    let nft2 = client.get_nft_info(&2);
    
    assert_eq!(nft1.owner, buyer1);
    assert_eq!(nft2.owner, buyer2);
    
    assert_eq!(nft1.transaction_id, txn_id1);
    assert_eq!(nft2.transaction_id, txn_id2);
    
    // Verify transaction mappings
    assert!(client.has_transaction_nft(&txn_id1));
    assert!(client.has_transaction_nft(&txn_id2));
    
    assert_eq!(client.get_nft_by_transaction(&txn_id1), Some(1));
    assert_eq!(client.get_nft_by_transaction(&txn_id2), Some(2));
}
