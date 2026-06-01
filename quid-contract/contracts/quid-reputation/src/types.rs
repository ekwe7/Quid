use soroban_sdk::{contracttype, Address};

/// On-chain reputation profile for a single user address.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationProfile {
    /// The wallet this profile belongs to.
    pub owner: Address,
    /// Number of successfully completed missions / approved submissions.
    pub success_count: u32,
    /// Number of rejected submissions.
    pub rejection_count: u32,
    /// Ledger timestamp of the last mutation.
    pub last_updated: u64,
}

/// Storage keys used by the reputation contract.
#[contracttype]
pub enum DataKey {
    /// Admin address that is allowed to mutate profiles.
    Admin,
    /// Per-user reputation profile, keyed by wallet address.
    Profile(Address),
}
