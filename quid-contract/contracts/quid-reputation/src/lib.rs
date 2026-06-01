#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env};

mod error;
mod types;

use error::ReputationError;
use types::{ContributorProfile, DataKey};

#[contractevent(topics = ["profile", "updated"])]
pub struct ProfileUpdatedEvent {
    pub subject: Address,
    pub accepted_submissions: u32,
    pub rejected_submissions: u32,
}

#[contract]
pub struct QuidReputationContract;

#[contractimpl]
impl QuidReputationContract {
    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Record a rejection for the given subject.
    /// Only the admin may call this.
    pub fn record_rejection(env: Env, subject: Address) -> Result<(), ReputationError> {
        // 1. Require admin auth before profile mutation
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        // 2. Load the subject profile or a default profile
        let key = DataKey::Profile(subject.clone());
        let mut profile: ContributorProfile = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(ContributorProfile {
                subject: subject.clone(),
                accepted_submissions: 0,
                rejected_submissions: 0,
                updated_at: 0,
            });

        // 3. Increment rejected_submissions, stamp updated_at, and store the record
        profile.rejected_submissions += 1;
        profile.updated_at = env.ledger().timestamp();

        env.storage().persistent().set(&key, &profile);
        env.storage()
            .persistent()
            .extend_ttl(&key, 5184000, 5184000);

        // Publish ProfileUpdatedEvent
        ProfileUpdatedEvent {
            subject,
            accepted_submissions: profile.accepted_submissions,
            rejected_submissions: profile.rejected_submissions,
        }
        .publish(&env);

        Ok(())
    }

    /// Get a contributor's profile.
    pub fn get_profile(env: Env, subject: Address) -> Option<ContributorProfile> {
        env.storage()
            .persistent()
            .get(&DataKey::Profile(subject))
    }

    fn get_admin(env: &Env) -> Result<Address, ReputationError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ReputationError::AdminNotSet)
    }
}

mod test;
