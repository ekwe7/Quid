#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use types::ReputationError;

fn setup() -> (Env, QuidReputationContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(QuidReputationContract, ());
    let client = QuidReputationContractClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_issue_and_get_attestation() {
    let (env, client) = setup();
    let issuer = Address::generate(&env);
    let recipient = Address::generate(&env);
    let cid = String::from_str(&env, "bafybeigdyrzt");

    let id = client.issue_attestation(&issuer, &recipient, &cid);
    assert_eq!(id, 1);

    let att = client.get_attestation(&id);
    assert_eq!(att.id, 1);
    assert_eq!(att.issuer, issuer);
    assert_eq!(att.recipient, recipient);
    assert_eq!(att.metadata_cid, cid);
}

#[test]
fn test_get_attestation_not_found() {
    let (_env, client) = setup();
    let result = client.try_get_attestation(&99);
    assert_eq!(
        result,
        Err(Ok(ReputationError::AttestationNotFound))
    );
}

#[test]
fn test_attestation_count() {
    let (env, client) = setup();
    let issuer = Address::generate(&env);
    let recipient = Address::generate(&env);
    let cid = String::from_str(&env, "bafybeigdyrzt");

    assert_eq!(client.get_attestation_count(), 0);
    client.issue_attestation(&issuer, &recipient, &cid);
    assert_eq!(client.get_attestation_count(), 1);
    client.issue_attestation(&issuer, &recipient, &cid);
    assert_eq!(client.get_attestation_count(), 2);
}
