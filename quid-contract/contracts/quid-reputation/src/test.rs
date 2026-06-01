#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

// ---------------------------------------------------------------------------
// Shared test environment
// ---------------------------------------------------------------------------

/// Registers the contract, mocks all auth, and returns a ready-to-use tuple
/// of `(env, contract_id, admin_address)`.
fn setup_env() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(QuidReputationContract, ());
    let admin = Address::generate(&env);

    (env, contract_id, admin)
}

// ---------------------------------------------------------------------------
// Admin bootstrap tests
// ---------------------------------------------------------------------------

#[test]
fn test_set_admin_succeeds_on_first_call() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // Bootstrap should succeed when no admin is set yet.
    client.set_admin(&admin);

    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_set_admin_fails_when_already_set() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    // A second call — even with a different address — must be rejected.
    let other = Address::generate(&env);
    client.set_admin(&other);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_get_admin_fails_when_not_set() {
    let (env, contract_id, _admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // No admin bootstrapped yet — should return NotAuthorized.
    client.get_admin();
}

// ---------------------------------------------------------------------------
// Profile upsert tests
// ---------------------------------------------------------------------------

#[test]
fn test_upsert_profile_creates_new_profile() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &5, &2);

    let profile = client.get_profile(&user);
    assert_eq!(profile.owner, user);
    assert_eq!(profile.success_count, 5);
    assert_eq!(profile.rejection_count, 2);
}

#[test]
fn test_upsert_profile_overwrites_existing_profile() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);

    // Initial upsert.
    client.upsert_profile(&user, &3, &1);

    // Overwrite with new values.
    client.upsert_profile(&user, &10, &4);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 10);
    assert_eq!(profile.rejection_count, 4);
}

#[test]
fn test_upsert_profile_stores_correct_owner() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &0, &0);

    let profile = client.get_profile(&user);
    assert_eq!(profile.owner, user);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_upsert_profile_requires_admin() {
    let (env, contract_id, _admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // No admin set — any mutation must fail with NotAuthorized.
    let user = Address::generate(&env);
    client.upsert_profile(&user, &1, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_get_profile_fails_when_not_found() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let unknown = Address::generate(&env);
    client.get_profile(&unknown);
}

// ---------------------------------------------------------------------------
// increment_success tests
// ---------------------------------------------------------------------------

#[test]
fn test_increment_success_increases_count_by_one() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &3, &1);

    client.increment_success(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 4);
    // rejection_count must remain unchanged.
    assert_eq!(profile.rejection_count, 1);
}

#[test]
fn test_increment_success_multiple_times() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &0, &0);

    client.increment_success(&user);
    client.increment_success(&user);
    client.increment_success(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 3);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_increment_success_fails_when_profile_missing() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let unknown = Address::generate(&env);
    client.increment_success(&unknown);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_increment_success_requires_admin() {
    let (env, contract_id, _admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // No admin bootstrapped — must fail.
    let user = Address::generate(&env);
    client.increment_success(&user);
}

// ---------------------------------------------------------------------------
// record_rejection tests
// ---------------------------------------------------------------------------

#[test]
fn test_record_rejection_increases_count_by_one() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &2, &0);

    client.record_rejection(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.rejection_count, 1);
    // success_count must remain unchanged.
    assert_eq!(profile.success_count, 2);
}

#[test]
fn test_record_rejection_multiple_times() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user = Address::generate(&env);
    client.upsert_profile(&user, &0, &0);

    client.record_rejection(&user);
    client.record_rejection(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.rejection_count, 2);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_record_rejection_fails_when_profile_missing() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let unknown = Address::generate(&env);
    client.record_rejection(&unknown);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_record_rejection_requires_admin() {
    let (env, contract_id, _admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // No admin bootstrapped — must fail.
    let user = Address::generate(&env);
    client.record_rejection(&user);
}

// ---------------------------------------------------------------------------
// Combined flow tests
// ---------------------------------------------------------------------------

#[test]
fn test_full_profile_lifecycle() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    // 1. Bootstrap admin.
    client.set_admin(&admin);
    assert_eq!(client.get_admin(), admin);

    // 2. Create a fresh profile.
    let user = Address::generate(&env);
    client.upsert_profile(&user, &0, &0);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 0);
    assert_eq!(profile.rejection_count, 0);

    // 3. Record two successes.
    client.increment_success(&user);
    client.increment_success(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 2);
    assert_eq!(profile.rejection_count, 0);

    // 4. Record one rejection.
    client.record_rejection(&user);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 2);
    assert_eq!(profile.rejection_count, 1);

    // 5. Upsert resets the counters.
    client.upsert_profile(&user, &10, &5);

    let profile = client.get_profile(&user);
    assert_eq!(profile.success_count, 10);
    assert_eq!(profile.rejection_count, 5);
}

#[test]
fn test_independent_profiles_do_not_interfere() {
    let (env, contract_id, admin) = setup_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.set_admin(&admin);

    let user_a = Address::generate(&env);
    let user_b = Address::generate(&env);

    client.upsert_profile(&user_a, &0, &0);
    client.upsert_profile(&user_b, &0, &0);

    // Mutate only user_a.
    client.increment_success(&user_a);
    client.increment_success(&user_a);
    client.record_rejection(&user_a);

    // user_b must be untouched.
    let profile_b = client.get_profile(&user_b);
    assert_eq!(profile_b.success_count, 0);
    assert_eq!(profile_b.rejection_count, 0);

    // user_a must reflect its own mutations.
    let profile_a = client.get_profile(&user_a);
    assert_eq!(profile_a.success_count, 2);
    assert_eq!(profile_a.rejection_count, 1);
}
