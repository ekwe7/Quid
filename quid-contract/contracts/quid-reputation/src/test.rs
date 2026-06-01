#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Events, Ledger};
use soroban_sdk::{Address, Env};

fn setup() -> (Env, Address, QuidReputationContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(QuidReputationContract, ());
    let client = QuidReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, admin, client)
}

#[test]
fn test_record_rejection_increments_count() {
    let (env, _admin, client) = setup();
    let subject = Address::generate(&env);

    // No profile yet
    assert!(client.get_profile(&subject).is_none());

    client.record_rejection(&subject);

    let profile = client.get_profile(&subject).unwrap();
    assert_eq!(profile.rejected_submissions, 1);
    assert_eq!(profile.accepted_submissions, 0);
}

#[test]
fn test_record_rejection_accumulates() {
    let (env, _admin, client) = setup();
    let subject = Address::generate(&env);

    client.record_rejection(&subject);
    client.record_rejection(&subject);
    client.record_rejection(&subject);

    let profile = client.get_profile(&subject).unwrap();
    assert_eq!(profile.rejected_submissions, 3);
}

#[test]
fn test_record_rejection_stamps_updated_at() {
    let (env, _admin, client) = setup();
    let subject = Address::generate(&env);

    env.ledger().set_timestamp(1000);
    client.record_rejection(&subject);

    let profile = client.get_profile(&subject).unwrap();
    assert_eq!(profile.updated_at, 1000);
}

#[test]
fn test_record_rejection_publishes_event() {
    let (env, _admin, client) = setup();
    let subject = Address::generate(&env);

    client.record_rejection(&subject);

    // Verify the event was published (non-empty events list)
    assert!(!env.events().all().is_empty());
}
