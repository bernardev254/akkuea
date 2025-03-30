#![cfg(test)]

use super::*;
use crate::datatype::{
    Category, CategoryRating, DataKey, Purchase, Review, ReviewError, ReviewStatus,
};
use crate::{AkkueaReviews, AkkueaReviewsClient};
use soroban_sdk::vec;
use soroban_sdk::IntoVal;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events, Ledger},
    Address, Env, String, Vec,
};

// Helper function to set up the test environment
fn setup_test() -> (
    Env,
    AkkueaReviewsClient<'static>,
    Address,
    Address,
    Address,
    u64,
) {
    let env = Env::default();
    let contract_id = env.register(AkkueaReviews {}, ());
    let client = AkkueaReviewsClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let payment_contract = Address::generate(&env);
    let user = Address::generate(&env);
    let product_id = 12345_u64;

    // Initialize the contract
    env.mock_all_auths();
    client.initialize(&admin, &payment_contract);

    (env, client, admin, payment_contract, user, product_id)
}

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let contract_id = env.register(AkkueaReviews {}, ());
    let client = AkkueaReviewsClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let payment_contract = Address::generate(&env);

    // First initialization should succeed
    env.mock_all_auths();
    client.initialize(&admin, &payment_contract);

    // Second initialization should fail
    env.mock_all_auths();
    let result = client.try_initialize(&admin, &payment_contract);
    assert!(result.is_err(), "Second initialization should fail");

    // Verify storage
    env.as_contract(&contract_id, || {
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Admin not set");
        let stored_payment: Address = env
            .storage()
            .persistent()
            .get(&DataKey::PaymentContract)
            .expect("Payment contract not set");
        assert_eq!(stored_admin, admin);
        assert_eq!(stored_payment, payment_contract);
    });
}

#[test]
fn test_record_purchase() {
    let (env, client, _admin, payment_contract, user, product_id) = setup_test();
    let purchase_link = String::from_str(&env, "https://example.com/purchase/123");

    // Record purchase
    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &Some(purchase_link.clone()));

    // Verify storage
    env.as_contract(&client.address, || {
        let purchase_key = DataKey::Purchase(user.clone(), product_id);
        let purchase: Purchase = env
            .storage()
            .persistent()
            .get(&purchase_key)
            .expect("Purchase not recorded");
        assert_eq!(purchase.user, user);
        assert_eq!(purchase.product_id, product_id);
        assert_eq!(purchase.purchase_time, env.ledger().timestamp());
        assert_eq!(purchase.review_id, None);
        assert_eq!(purchase.purchase_link, Some(purchase_link.clone()));
    });

    // Test duplicate purchase fails
    env.mock_all_auths();
    let result = client.try_record_purchase(&user, &product_id, &Some(purchase_link));
    assert!(result.is_err(), "Duplicate purchase should fail");
}

#[test]
fn test_submit_review() {
    let (env, client, _admin, _, user, product_id) = setup_test();

    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);

    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    let review_text = String::from_str(&env, "Great product!");
    let multimedia = Vec::new(&env);

    env.mock_all_auths();
    let review_id = client.submit_review(
        &user,
        &product_id,
        &category_ratings,
        &Some(review_text.clone()),
        &multimedia,
    );

    env.as_contract(&client.address, || {
        let review_key = DataKey::Review(product_id, review_id);
        let review: Review = env
            .storage()
            .persistent()
            .get(&review_key)
            .expect("Review not stored");
        assert_eq!(review.reviewer, user);
        assert_eq!(review.text, Some(review_text));
        assert_eq!(review.category_ratings.len(), 1);
        assert_eq!(review.status, ReviewStatus::Verified);
    });

    let events = env.events().all();
    //println!("Events: {:?}", events); // Debug output
    assert_eq!(
        events.len(),
        3,
        "Should emit init, purchase, and review events"
    );
}

#[test]
fn test_submit_review_events() {
    let env = Env::default();
    let contract_id = env.register(AkkueaReviews {}, ());
    let client = AkkueaReviewsClient::new(&env, &contract_id);

    // Set up admin and payment contract
    let admin = Address::generate(&env);
    let payment_contract = Address::generate(&env);
    let user = Address::generate(&env);
    let product_id = 12345_u64;

    // Initialize contract
    env.mock_all_auths();
    client.initialize(&admin, &payment_contract);

    // Record purchase
    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);

    // Submit review
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    let review_text = String::from_str(&env, "Great product!");
    let multimedia = Vec::new(&env);

    env.mock_all_auths();
    let review_id = client.submit_review(
        &user,
        &product_id,
        &category_ratings,
        &Some(review_text.clone()),
        &multimedia,
    );

    // Verify review ID
    assert_eq!(review_id, 0, "First review should have ID 0");

    // // Verify events
    // assert_eq!(
    //     env.events().all(),
    //     vec![
    //         &env,
    //         // Initialization event
    //         (
    //             contract_id.clone(),
    //             vec![&env, Symbol::new(&env, "init"), admin],
    //             env.ledger().timestamp().into_val(&env)
    //         ),
    //         // Purchase recorded event
    //         (
    //             contract_id.clone(),
    //             vec![&env, Symbol::new(&env, "pur_rec"), user],
    //             product_id.into_val(&env)
    //         ),
    //         // Review submitted event (matches your Symbol::new usage)
    //         (
    //             contract_id,
    //             vec![&env, Symbol::new(&env, "review_submitted"), user],
    //             vec![&env, product_id, review_id, 4_u64].into_val(&env) // FourStars = 4
    //         ),
    //     ]
    // );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")] // Assuming PurchaseNotFound = #2
fn test_submit_review_no_purchase() {
    let (env, client, _admin, _payment_contract, user, product_id) = setup_test();

    // Attempt to submit review without purchase
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    env.mock_all_auths();
    client.submit_review(
        &user,
        &product_id,
        &category_ratings,
        &None,
        &Vec::new(&env),
    );
}

#[test]
fn test_review_window_expiration() {
    let (env, client, _admin, payment_contract, user, product_id) = setup_test();

    // Record purchase
    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);

    // Advance ledger time beyond review window (30 days + 1 second)
    env.ledger()
        .set_timestamp(env.ledger().timestamp() + crate::REVIEW_WINDOW + 1);

    // Attempt to submit review
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    env.mock_all_auths();
    let result = client.try_submit_review(
        &user,
        &product_id,
        &category_ratings,
        &None,
        &Vec::new(&env),
    );
    assert!(result.is_err(), "Review should fail after window expires");
}

// In lib.rs, ReviewOperations impl
fn add_response(
    env: Env,
    author: Address,
    product_id: u64,
    review_id: u32,
    response_text: String,
) -> Result<(), ReviewError> {
    author.require_auth();

    if response_text.len() > MAX_TEXT_LENGTH {
        return Err(ReviewError::TextTooLong);
    }

    let review_key = DataKey::Review(product_id, review_id);
    let mut review: Review = env
        .storage()
        .persistent()
        .get(&review_key)
        .ok_or(ReviewError::ReviewNotFound)?;

    let owner_key = DataKey::ProductOwner(product_id);
    let product_owner = env
        .storage()
        .persistent()
        .get(&owner_key)
        .ok_or(ReviewError::ProductNotFound)?;

    if author != product_owner {
        let purchase_key = DataKey::Purchase(author.clone(), product_id);
        let purchase: Purchase = env
            .storage()
            .persistent()
            .get(&purchase_key)
            .ok_or(ReviewError::PurchaseNotFound)?;
        if purchase.review_id != Some(review_id) {
            return Err(ReviewError::Unauthorized);
        }
    }

    let response = Response {
        author: author.clone(),
        text: response_text.clone(),
        timestamp: env.ledger().timestamp(),
    };
    review.responses.push_back(response);
    env.storage().persistent().set(&review_key, &review);

    env.events().publish(
        (Symbol::new(&env, "response_added"), author),
        (product_id, review_id, response_text),
    );

    Ok(())
}

#[test]
fn test_vote_helpful() {
    let (env, client, _admin, _, user, product_id) = setup_test();

    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    let review_id = client.submit_review(
        &user,
        &product_id,
        &category_ratings,
        &None,
        &Vec::new(&env),
    );

    let voter = Address::generate(&env);
    env.mock_all_auths();
    client.vote_helpful(&voter, &product_id, &review_id, &true);

    let events = env.events().all();
    //println!("Events: {:?}", events); // Debug output
    assert_eq!(
        events.len(),
        4,
        "Should emit init, purchase, review, and vote events"
    );
}

#[test]
fn test_dispute_review() {
    let env = Env::default();
    let contract_id = env.register(AkkueaReviews {}, ()); // Updated to env.register
    let client = AkkueaReviewsClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let product_id = 12345u64;
    let review_id = 0u32;

    env.mock_all_auths();
    client.initialize(&admin, &Address::generate(&env));
    client.record_purchase(&user, &product_id, &None);
    client.submit_review(&user, &product_id, &vec![&env], &None, &vec![&env]);

    env.mock_all_auths();
    let dispute_id = client.dispute_review(&product_id, &review_id); // Now returns u32
    let review = client.get_review(&product_id, &review_id);
    assert_eq!(review.status, ReviewStatus::Disputed);
    assert_eq!(review.dispute_id, Some(dispute_id));
}

#[test]
fn test_resolve_dispute() {
    let env = Env::default();
    let contract_id = env.register(AkkueaReviews {}, ()); // Updated to env.register
    let client = AkkueaReviewsClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let product_id = 12345u64;
    let review_id = 0u32;

    env.mock_all_auths();
    client.initialize(&admin, &Address::generate(&env));
    client.record_purchase(&user, &product_id, &None);
    client.submit_review(&user, &product_id, &vec![&env], &None, &vec![&env]);
    let dispute_id = client.dispute_review(&product_id, &review_id); // Now returns u32

    env.mock_all_auths();
    client.resolve_dispute(&dispute_id);

    let review = client.get_review(&product_id, &review_id);
    assert_eq!(review.status, ReviewStatus::Verified);
}

#[test]
fn test_max_text_length() {
    let (env, client, _admin, payment_contract, user, product_id) = setup_test();

    // Record purchase
    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);

    // Test max text length (500 chars)
    let max_text = String::from_str(&env, &"a".repeat(500));
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    env.mock_all_auths();
    client.submit_review(
        &user,
        &product_id,
        &category_ratings,
        &Some(max_text),
        &Vec::new(&env),
    );

    // Test exceeding max text length (501 chars)
    let too_long_text = String::from_str(&env, &"a".repeat(501));
    env.mock_all_auths();
    let result = client.try_submit_review(
        &user,
        &product_id,
        &category_ratings,
        &Some(too_long_text),
        &Vec::new(&env),
    );
    assert!(result.is_err(), "Text exceeding 500 chars should fail");
}

#[test]
fn test_max_multimedia_limit() {
    let (env, client, _admin, payment_contract, user, product_id) = setup_test();

    // Record purchase
    env.mock_all_auths();
    client.record_purchase(&user, &product_id, &None);

    // Test max multimedia limit (5 items)
    let multimedia = vec![
        &env,
        String::from_str(&env, "img1.jpg"),
        String::from_str(&env, "img2.jpg"),
        String::from_str(&env, "img3.jpg"),
        String::from_str(&env, "img4.jpg"),
        String::from_str(&env, "img5.jpg"),
    ];
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    env.mock_all_auths();
    client.submit_review(&user, &product_id, &category_ratings, &None, &multimedia);

    // Test exceeding max multimedia limit (6 items)
    let too_many_multimedia = vec![
        &env,
        String::from_str(&env, "img1.jpg"),
        String::from_str(&env, "img2.jpg"),
        String::from_str(&env, "img3.jpg"),
        String::from_str(&env, "img4.jpg"),
        String::from_str(&env, "img5.jpg"),
        String::from_str(&env, "img6.jpg"),
    ];
    env.mock_all_auths();
    let result = client.try_submit_review(
        &user,
        &product_id,
        &category_ratings,
        &None,
        &too_many_multimedia,
    );
    assert!(result.is_err(), "More than 5 multimedia items should fail");
}

#[test]
fn test_boundary_conditions() {
    let (env, client, _admin, payment_contract, user, product_id) = setup_test();

    // Test max product ID
    let max_product_id = u64::MAX;
    env.mock_all_auths();
    client.record_purchase(&user, &max_product_id, &None);
    let category_ratings = vec![
        &env,
        CategoryRating {
            category: Category::ContentQuality,
            rating: crate::datatype::Rating::FourStars,
            timestamp: env.ledger().timestamp(),
        },
    ];
    env.mock_all_auths();
    client.submit_review(
        &user,
        &max_product_id,
        &category_ratings,
        &None,
        &Vec::new(&env),
    );

    // Test min/max timestamps
    env.ledger().set_timestamp(0);
    env.mock_all_auths();
    client.record_purchase(&user, &(product_id + 1), &None);
    env.mock_all_auths();
    client.submit_review(
        &user,
        &(product_id + 1),
        &category_ratings,
        &None,
        &Vec::new(&env),
    );

    env.ledger().set_timestamp(u64::MAX);
    env.mock_all_auths();
    client.record_purchase(&user, &(product_id + 2), &None);
    env.mock_all_auths();
    client.submit_review(
        &user,
        &(product_id + 2),
        &category_ratings,
        &None,
        &Vec::new(&env),
    );
}
