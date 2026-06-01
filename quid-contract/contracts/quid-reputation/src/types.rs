use soroban_sdk::{contracttype, contracterror, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub recipient: Address,
    pub metadata_cid: String,
    pub issued_at: u64,
}

#[contracttype]
pub enum DataKey {
    Attestation(u64),
    AttestationCount,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReputationError {
    AttestationNotFound = 1,
}
