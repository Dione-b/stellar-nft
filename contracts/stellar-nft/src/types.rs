use soroban_sdk::{contracttype, String};

#[contracttype]
pub enum DataKey {
    Owner,
    TotalMinted,
    MaxSupply,
    TokenData(u32),
}

#[contracttype]
pub struct TokenData {
    pub session_id: String,
    pub resource: String,
}

#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}