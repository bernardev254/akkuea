use crate::educator_contract::{self, Client as EducatorClient};
use crate::{TippingRewards, TippingRewardsClient};
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String, Vec,
};
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};




#[test]
fn test_send_and_track_tips() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let educator = Address::generate(&env);
    let tipper = Address::generate(&env);

    // Deploy the Stellar Asset Contract
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());

    // Get the contract address
    let token_address = token_contract.address();

    // Create the token client (for standard token operations)
    let token = TokenClient::new(&env, &token_address);

    // Create the Stellar Asset specific client (for SAC-specific operations like mint)
    let token_sac = StellarAssetClient::new(&env, &token_address);

    env.mock_all_auths();
    // Mint tokens to the tipper for testing
    token_sac.mint(&tipper, &1000);
    assert_eq!(token.balance(&tipper), 1000);

    // Deploy educator contract properly
    let educator_contract_id = env.register(educator_contract::WASM, ());
    let educator_client = EducatorClient::new(&env, &educator_contract_id);

    // Initialize educator contract with admin
    educator_client.initialize(&admin);

    // Register educator in educator contract
    educator_client.register_educator(
        &educator,
        &String::from_str(&env, "Test Educator"),
        &Vec::new(&env),
        &Vec::new(&env),
    );

    // Deploy tipping contract
    let contract_id = env.register(TippingRewards, (&educator_contract_id,));
    let client = TippingRewardsClient::new(&env, &contract_id);

    // Send a tip
    client.send_tip(&token_address, &tipper, &educator, &100);

    // Verify tip history
    let tips_sent = client.get_tips_sent(&tipper);
    let tips_received = client.get_tips_received(&educator);

    assert_eq!(tips_sent.len(), 1);
    assert_eq!(tips_received.len(), 1);
}
