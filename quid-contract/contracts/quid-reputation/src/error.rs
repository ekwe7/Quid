use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReputationError {
    NotAuthorized = 1,
    AdminNotSet = 2,
}
