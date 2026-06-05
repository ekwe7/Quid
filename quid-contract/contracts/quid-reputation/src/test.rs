#![cfg(test)]

use crate::{types::Profile, QuidReputationContract, QuidReputationContractClient};
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, String,
};

fn setup_test_env() -> (Env, Address, Address) {
    let env = Env::default();
    let admin = Address::random(&env);

    let result = QuidReputationContract::bootstrap_admin(env.clone(), admin.clone());

    assert!(result.is_ok());

    let retrieved_admin = QuidReputationContract::get_admin(env).unwrap();
    assert_eq!(retrieved_admin, admin);
}

#[test]
fn test_bootstrap_admin() {
    let env = Env::default();
    let admin = Address::random(&env);

    let result = QuidReputationContract::bootstrap_admin(env.clone(), admin.clone());

    assert!(result.is_ok());

    let retrieved_admin = QuidReputationContract::get_admin(env).unwrap();
    assert_eq!(retrieved_admin, admin);
}

#[test]
fn test_bootstrap_admin_twice_fails() {
    let env = Env::default();
    let admin1 = Address::random(&env);
    let admin2 = Address::random(&env);

    QuidReputationContract::bootstrap_admin(env.clone(), admin1).unwrap();
    let result = QuidReputationContract::bootstrap_admin(env, admin2);

    assert!(result.is_err());
}

#[test]
fn test_issue_attestation() {
    let env = Env::default();
    let issuer = Address::random(&env);
    let subject = Address::random(&env);

    let attestation_id = QuidReputationContract::issue_attestation(
        env.clone(),
        issuer.clone(),
        subject.clone(),
        String::from_slice(&env, "contributor"),
        String::from_slice(&env, "Active contributor"),
        Some(String::from_slice(&env, "QmExample123")),
        None,
    )
    .unwrap();

    assert_eq!(attestation_id, 1);

    let attestation = QuidReputationContract::get_attestation(env, attestation_id).unwrap();
    assert_eq!(attestation.issuer, issuer);
    assert_eq!(attestation.subject, subject);
    assert_eq!(attestation.revoked, false);
}

#[test]
fn test_issue_attestation_with_expiry() {
    let env = Env::default();
    let issuer = Address::random(&env);
    let subject = Address::random(&env);

    let now = env.ledger().timestamp();
    let expiry = now + 86400; // 1 day from now

    let attestation_id = QuidReputationContract::issue_attestation(
        env.clone(),
        issuer.clone(),
        subject.clone(),
        String::from_slice(&env, "expert"),
        String::from_slice(&env, "Expert reviewer"),
        None,
        Some(expiry),
    )
    .unwrap();

    let attestation = QuidReputationContract::get_attestation(env, attestation_id).unwrap();
    assert_eq!(attestation.expires_at, Some(expiry));
}

#[test]
fn test_revoke_attestation() {
    let env = Env::default();
    let issuer = Address::random(&env);
    let subject = Address::random(&env);

    let attestation_id = QuidReputationContract::issue_attestation(
        env.clone(),
        issuer.clone(),
        subject.clone(),
        String::from_slice(&env, "contributor"),
        String::from_slice(&env, "Active contributor"),
        None,
        None,
    )
    .unwrap();

    let result = QuidReputationContract::revoke_attestation(env.clone(), attestation_id);
    assert!(result.is_ok());

    let attestation = QuidReputationContract::get_attestation(env, attestation_id).unwrap();
    assert_eq!(attestation.revoked, true);
}

#[test]
fn test_empty_label_fails() {
    let env = Env::default();
    let issuer = Address::random(&env);
    let subject = Address::random(&env);

    let result = QuidReputationContract::issue_attestation(
        env,
        issuer,
        subject,
        String::from_slice(&env, "contributor"),
        String::from_slice(&env, ""),
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn test_invalid_expiry_time_fails() {
    let env = Env::default();
    let issuer = Address::random(&env);
    let subject = Address::random(&env);

    let attestation_id = client.issue_attestation(&issuer, &subject, &attestation_type, &data_cid);

    assert!(client.attestation_exists(&attestation_id));
    let now = env.ledger().timestamp();
    let past_expiry = now - 1000; // In the past

    let result = QuidReputationContract::issue_attestation(
        env,
        issuer,
        subject,
        String::from_slice(&env, "contributor"),
        String::from_slice(&env, "Active contributor"),
        None,
        Some(past_expiry),
    );

    assert!(result.is_err());
}

<<<<<<<<< Temporary merge branch 1
#[test]
fn test_create_and_get_profile() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);
    let updated_at = env.ledger().timestamp();

    let profile = Profile {
        subject: subject.clone(),
        successful_missions: 5,
        rejected_submissions: 2,
        reviewer_score: 100,
        founder_score: 50,
        total_earnings: 1000,
        updated_at,
    };

    client.set_profile(&profile);

    let retrieved_profile = client.get_profile(&subject);
    assert_eq!(retrieved_profile.subject, subject);
    assert_eq!(retrieved_profile.successful_missions, 5);
    assert_eq!(retrieved_profile.rejected_submissions, 2);
    assert_eq!(retrieved_profile.reviewer_score, 100);
    assert_eq!(retrieved_profile.founder_score, 50);
    assert_eq!(retrieved_profile.total_earnings, 1000);
    assert_eq!(retrieved_profile.updated_at, updated_at);
}

#[test]
fn test_update_profile() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);

    let profile = Profile {
        subject: subject.clone(),
        successful_missions: 5,
        rejected_submissions: 2,
        reviewer_score: 100,
        founder_score: 50,
        total_earnings: 1000,
        updated_at: env.ledger().timestamp(),
    };

    client.set_profile(&profile);

    // Update the profile
    let updated_profile = Profile {
        subject: subject.clone(),
        successful_missions: 10,
        rejected_submissions: 3,
        reviewer_score: 150,
        founder_score: 75,
        total_earnings: 2000,
        updated_at: env.ledger().timestamp(),
    };

    client.set_profile(&updated_profile);

    let retrieved_profile = client.get_profile(&subject);
    assert_eq!(retrieved_profile.successful_missions, 10);
    assert_eq!(retrieved_profile.rejected_submissions, 3);
    assert_eq!(retrieved_profile.reviewer_score, 150);
    assert_eq!(retrieved_profile.founder_score, 75);
    assert_eq!(retrieved_profile.total_earnings, 2000);
}

#[test]
fn test_profile_exists() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);

    assert!(!client.profile_exists(&subject));

    let profile = Profile {
        subject: subject.clone(),
        successful_missions: 0,
        rejected_submissions: 0,
        reviewer_score: 0,
        founder_score: 0,
        total_earnings: 0,
        updated_at: env.ledger().timestamp(),
    };

    client.set_profile(&profile);

    assert!(client.profile_exists(&subject));
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_get_profile_not_found() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);
    client.get_profile(&subject);
=========
// -------------------------------------------------------------------------
// Profile helper tests
// -------------------------------------------------------------------------

#[test]
fn test_get_profile_not_found() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let subject = Address::generate(&env);
    let result = client.try_get_profile(&subject);
    assert_eq!(result, Err(Ok(ReputationError::ProfileNotFound)));
}

#[test]
fn test_store_and_get_profile() {
    let (env, contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &contract_id);

    let subject = Address::generate(&env);

    env.as_contract(&contract_id, || {
        let profile = Profile {
            subject: subject.clone(),
            score: 42,
            missions_completed: 3,
            missions_created: 1,
        };
        QuidReputationContract::store_profile(&env, &profile);
    });

    let fetched = client.get_profile(&subject);
    assert_eq!(fetched.score, 42);
    assert_eq!(fetched.missions_completed, 3);
    assert_eq!(fetched.missions_created, 1);
}

#[test]
fn test_load_or_default_returns_zeroed_profile() {
    let (env, contract_id, _admin) = setup_test_env();

    let subject = Address::generate(&env);

    env.as_contract(&contract_id, || {
        let profile = QuidReputationContract::load_or_default(&env, subject.clone());
        assert_eq!(profile.subject, subject);
        assert_eq!(profile.score, 0);
        assert_eq!(profile.missions_completed, 0);
        assert_eq!(profile.missions_created, 0);
    });
}

#[test]
fn test_create_and_get_profile() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);

    let profile = Profile {
        subject: subject.clone(),
        score: 150,
        missions_completed: 5,
        missions_created: 2,
    };

    client.set_profile(&profile);

    let retrieved_profile = client.get_profile(&subject);
    assert_eq!(retrieved_profile.subject, subject);
    assert_eq!(retrieved_profile.score, 150);
    assert_eq!(retrieved_profile.missions_completed, 5);
    assert_eq!(retrieved_profile.missions_created, 2);
}

#[test]
fn test_update_profile() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);

    let profile = Profile {
        subject: subject.clone(),
        score: 100,
        missions_completed: 5,
        missions_created: 2,
    };

    client.set_profile(&profile);

    // Update the profile
    let updated_profile = Profile {
        subject: subject.clone(),
        score: 225,
        missions_completed: 10,
        missions_created: 3,
    };

    client.set_profile(&updated_profile);

    let retrieved_profile = client.get_profile(&subject);
    assert_eq!(retrieved_profile.score, 225);
    assert_eq!(retrieved_profile.missions_completed, 10);
    assert_eq!(retrieved_profile.missions_created, 3);
}

#[test]
fn test_profile_exists() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);

    assert!(!client.profile_exists(&subject));

    let profile = Profile {
        subject: subject.clone(),
        score: 0,
        missions_completed: 0,
        missions_created: 0,
    };

    client.set_profile(&profile);

    assert!(client.profile_exists(&subject));
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_get_profile_not_found() {
    let (env, _contract_id, _admin) = setup_test_env();
    let client = QuidReputationContractClient::new(&env, &_contract_id);

    let subject = Address::generate(&env);
    client.get_profile(&subject);
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

    // Revoke the attestation
    client.revoke_attestation(&issuer, &attestation_id);

    // Verify the event was published
    let events = env.events().all();
    let event = events.last().unwrap();

    // Check that the event contains the attestation_id and revoked_by
    assert_eq!(event.0, contract_id);

    // The event topics should contain "attestation" and "revoked"
    let topics = event.1.clone();
    assert_eq!(topics.len(), 2);
}
