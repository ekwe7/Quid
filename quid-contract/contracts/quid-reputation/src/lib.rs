#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod types;
use types::{Attestation, DataKey, ReputationError};

#[contract]
pub struct QuidReputationContract;

#[contractimpl]
impl QuidReputationContract {
    /// Issue a new attestation and return its id.
    pub fn issue_attestation(
        env: Env,
        issuer: Address,
        recipient: Address,
        metadata_cid: String,
    ) -> u64 {
        issuer.require_auth();

        let id = Self::next_id(&env);

        let attestation = Attestation {
            id,
            issuer,
            recipient,
            metadata_cid,
            issued_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Attestation(id), &attestation);

        id
    }

    /// Fetch an attestation by id.
    pub fn get_attestation(env: Env, attestation_id: u64) -> Result<Attestation, ReputationError> {
        env.storage()
            .persistent()
            .get(&DataKey::Attestation(attestation_id))
            .ok_or(ReputationError::AttestationNotFound)
    }

    /// Return the total number of issued attestations.
    pub fn get_attestation_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::AttestationCount)
            .unwrap_or(0)
    }

    fn next_id(env: &Env) -> u64 {
        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::AttestationCount)
            .unwrap_or(0);
        count += 1;
        env.storage()
            .instance()
            .set(&DataKey::AttestationCount, &count);
        count
    }
}

mod test;
