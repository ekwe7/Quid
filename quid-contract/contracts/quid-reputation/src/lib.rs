#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod error;
mod types;

use error::ReputationError;
use types::{DataKey, ReputationProfile};

#[contract]
pub struct QuidReputationContract;

#[contractimpl]
impl QuidReputationContract {
    // -----------------------------------------------------------------------
    // Admin bootstrap
    // -----------------------------------------------------------------------

    /// Bootstrap the contract admin.  Can only be called once; subsequent
    /// calls return `AdminAlreadySet`.
    pub fn set_admin(env: Env, admin: Address) -> Result<(), ReputationError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ReputationError::AdminAlreadySet);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Return the current admin address.
    pub fn get_admin(env: Env) -> Result<Address, ReputationError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ReputationError::NotAuthorized)
    }

    // -----------------------------------------------------------------------
    // Profile management
    // -----------------------------------------------------------------------

    /// Create or fully replace a reputation profile for `owner`.
    /// Only the admin may call this.
    pub fn upsert_profile(
        env: Env,
        owner: Address,
        success_count: u32,
        rejection_count: u32,
    ) -> Result<(), ReputationError> {
        Self::require_admin(&env)?;

        let profile = ReputationProfile {
            owner: owner.clone(),
            success_count,
            rejection_count,
            last_updated: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Profile(owner.clone()), &profile);

        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Profile(owner), 5_184_000, 5_184_000);

        Ok(())
    }

    /// Fetch the reputation profile for `owner`.
    pub fn get_profile(env: Env, owner: Address) -> Result<ReputationProfile, ReputationError> {
        env.storage()
            .persistent()
            .get(&DataKey::Profile(owner))
            .ok_or(ReputationError::ProfileNotFound)
    }

    // -----------------------------------------------------------------------
    // Mutation helpers
    // -----------------------------------------------------------------------

    /// Increment the `success_count` of an existing profile by one.
    /// Only the admin may call this.
    pub fn increment_success(env: Env, owner: Address) -> Result<(), ReputationError> {
        Self::require_admin(&env)?;

        let mut profile = Self::get_profile(env.clone(), owner.clone())?;
        profile.success_count = profile.success_count.saturating_add(1);
        profile.last_updated = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&DataKey::Profile(owner.clone()), &profile);

        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Profile(owner), 5_184_000, 5_184_000);

        Ok(())
    }

    /// Increment the `rejection_count` of an existing profile by one.
    /// Only the admin may call this.
    pub fn record_rejection(env: Env, owner: Address) -> Result<(), ReputationError> {
        Self::require_admin(&env)?;

        let mut profile = Self::get_profile(env.clone(), owner.clone())?;
        profile.rejection_count = profile.rejection_count.saturating_add(1);
        profile.last_updated = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&DataKey::Profile(owner.clone()), &profile);

        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Profile(owner), 5_184_000, 5_184_000);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn require_admin(env: &Env) -> Result<(), ReputationError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ReputationError::NotAuthorized)?;

        admin.require_auth();
        Ok(())
    }
}

mod test;
