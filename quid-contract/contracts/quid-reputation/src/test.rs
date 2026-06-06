#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_test_env() -> (Env, Address, QuidReputationContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(QuidReputationContract, ());
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.bootstrap_admin(&admin);

    (env, admin, client)
}

fn setup_test_env() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register(QuidReputationContract, ());
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.bootstrap_admin(&admin);

    let retrieved_admin = client.get_admin();
    assert_eq!(retrieved_admin, admin);
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let contract_id = env.register(QuidReputationContract, ());
    let client = QuidReputationContractClient::new(&env, &contract_id);

    client.bootstrap_admin(&admin1);
    let result = client.try_bootstrap_admin(&admin2);

    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_issue_attestation() {
    let (env, _admin, client) = setup_test_env();
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_id = client.issue_attestation(
        &issuer,
        &subject,
        &String::from_str(&env, "contributor"),
        &String::from_str(&env, "Active contributor"),
        &Some(String::from_str(&env, "QmExample123")),
        &None,
    );

    assert_eq!(attestation_id, 1);

    let attestation = client.get_attestation(&attestation_id);
    assert_eq!(attestation.issuer, issuer);
    assert_eq!(attestation.subject, subject);
    assert_eq!(attestation.attestation_type, attestation_type);
    assert_eq!(attestation.data_cid, data_cid);
    assert!(!attestation.revoked);
}

#[test]
fn test_revoke_attestation_by_issuer() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    client.revoke_attestation(&issuer, &attestation_id);

    let attestation = client.get_attestation(&attestation_id);
    assert!(attestation.revoked);
}

#[test]
fn test_revoke_attestation_by_admin() {
    let (env, contract_id, admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    client.revoke_attestation(&admin, &attestation_id);

    let attestation = client.get_attestation(&attestation_id);
    assert!(attestation.revoked);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_revoke_attestation_unauthorized() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let unauthorized = Address::generate(&env);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    client.revoke_attestation(&unauthorized, &attestation_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_revoke_already_revoked_attestation() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    client.revoke_attestation(&issuer, &attestation_id);
    client.revoke_attestation(&issuer, &attestation_id);
}

#[test]
fn test_attestation_count() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    assert_eq!(client.get_attestation_count(), 0);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);
    assert_eq!(client.get_attestation_count(), 1);

    client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);
    assert_eq!(client.get_attestation_count(), 2);
}

#[test]
fn test_attestation_exists() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    assert!(!client.attestation_exists(&1));

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    assert!(client.attestation_exists(&attestation_id));
}

#[test]
fn test_revoke_attestation_publishes_event() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_type = String::from_str(&env, "skill");
    let data_cid = String::from_str(&env, "QmTest123");

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    client.revoke_attestation(&issuer, &attestation_id);

    let events = env.events().all();
    let event = events.last().unwrap();

    assert_eq!(event.0, contract_id);

    let topics = event.1.clone();
    assert_eq!(topics.len(), 2);
    assert!(!attestation.revoked);
}

#[test]
fn test_issue_attestation_with_expiry() {
    let (env, _admin, client) = setup_test_env();
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let now = env.ledger().timestamp();
    let expiry = now + 86400; // 1 day from now

    let attestation_id = client.issue_attestation(
        &issuer,
        &subject,
        &String::from_str(&env, "expert"),
        &String::from_str(&env, "Expert reviewer"),
        &None,
        &Some(expiry),
    );

    let attestation = client.get_attestation(&attestation_id);
    assert_eq!(attestation.expires_at, Some(expiry));
}

#[test]
fn test_revoke_attestation() {
    let (env, _admin, client) = setup_test_env();
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let attestation_id = client.issue_attestation(
        &issuer,
        &subject,
        &String::from_str(&env, "contributor"),
        &String::from_str(&env, "Active contributor"),
        &None,
        &None,
    );

    client.revoke_attestation(&attestation_id);

    let attestation = client.get_attestation(&attestation_id);
    assert!(attestation.revoked);
}

#[test]
fn test_empty_label_fails() {
    let (env, _admin, client) = setup_test_env();
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let result = client.try_issue_attestation(
        &issuer,
        &subject,
        &String::from_str(&env, "contributor"),
        &String::from_str(&env, ""),
        &None,
        &None,
    );

    assert!(result.is_err());
}

#[test]
fn test_invalid_expiry_time_fails() {
    let (env, _admin, client) = setup_test_env();
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let now = env.ledger().timestamp();
    let past_expiry = now; // Expiry <= now should fail

    let result = client.try_issue_attestation(
        &issuer,
        &subject,
        &String::from_str(&env, "contributor"),
        &String::from_str(&env, "Active contributor"),
        &None,
        &Some(past_expiry),
    );

    assert!(result.is_err());
}

// -------------------------------------------------------------------------
// Profile helper tests
// -------------------------------------------------------------------------

#[test]
fn test_get_profile_not_found() {
    let (env, _admin, client) = setup_test_env();

    let subject = Address::generate(&env);
    let result = client.try_get_profile(&subject);
    assert_eq!(result, Err(Ok(QuidError::ProfileNotFound)));
}

#[test]
fn test_store_and_get_profile() {
    let (env, _admin, client) = setup_test_env();
    let subject = Address::generate(&env);

    client.increment_success(&subject, &0); // ensure profile exists

    let fetched = client.get_profile(&subject);
    assert_eq!(fetched.score, 0);
    assert_eq!(fetched.successful_missions, 1);
    assert_eq!(fetched.missions_created, 0);
    assert_eq!(fetched.total_earnings, 0);
}

#[test]
fn test_load_or_default_returns_zeroed_profile() {
    let (env, _admin, client) = setup_test_env();

    let subject = Address::generate(&env);

    // If profile doesn't exist, it should return error.
    let result = client.try_get_profile(&subject);
    assert_eq!(result, Err(Ok(QuidError::ProfileNotFound)));
}

// -------------------------------------------------------------------------
// increment_success tests
// -------------------------------------------------------------------------

#[test]
fn test_increment_success() {
    let (env, _admin, client) = setup_test_env();

    let subject = Address::generate(&env);

    // First increment
    client.increment_success(&subject, &100);

    let fetched = client.get_profile(&subject);
    assert_eq!(fetched.successful_missions, 1);
    assert_eq!(fetched.total_earnings, 100);

    // Second increment
    client.increment_success(&subject, &50);

    let fetched2 = client.get_profile(&subject);
    assert_eq!(fetched2.successful_missions, 2);
    assert_eq!(fetched2.total_earnings, 150);
}

#[test]
fn test_increment_success_invalid_reward() {
    let (env, _admin, client) = setup_test_env();

    let subject = Address::generate(&env);

    let res = client.try_increment_success(&subject, &-10);
    assert_eq!(res, Err(Ok(QuidError::InvalidRewardAmount)));
}
