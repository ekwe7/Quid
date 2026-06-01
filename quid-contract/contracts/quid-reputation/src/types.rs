use soroban_sdk::{contracttype, Address};

#[contracttype]
pub enum DataKey {
    Profile(Address),
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributorProfile {
    pub subject: Address,
    pub accepted_submissions: u32,
    pub rejected_submissions: u32,
    pub updated_at: u64,
}
