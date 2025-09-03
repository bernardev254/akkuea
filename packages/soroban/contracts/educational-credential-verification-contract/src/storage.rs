use soroban_sdk::{contracttype, symbol_short, Address, Symbol, BytesN};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    ReviewCounter(Address),
    Reviews(Address),
    Credential(BytesN<32>),
    NFT(BytesN<32>),
    Template(u32),
    Badge(BytesN<32>),
}

// Existing storage keys
pub const ADMIN: Symbol = symbol_short!("admin");
pub const REVIEWERS: Symbol = symbol_short!("reviewers");
pub const EDUCATORS: Symbol = symbol_short!("EDU");
pub const DISPUTES: Symbol = symbol_short!("disputes");
pub const ANALYTICS: Symbol = symbol_short!("analytics");
pub const REVOKED: Symbol = symbol_short!("REVOKE");
pub const VERIFIED_CREDS: Symbol = symbol_short!("vcreds");
pub const SIGNATURES: Symbol = symbol_short!("sigs");
pub const AUTH_INST: Symbol = symbol_short!("AUTH_INST");

// New storage keys for enhanced features
pub const CREDENTIALS: Symbol = symbol_short!("CREDS");
pub const NFTS: Symbol = symbol_short!("NFTS");
pub const NFT_TEMPLATES: Symbol = symbol_short!("TEMPLATES");
pub const ACHIEVEMENT_BADGES: Symbol = symbol_short!("BADGES");
pub const CREDENTIAL_COUNTER: Symbol = symbol_short!("CRED_CTR");
pub const NFT_COUNTER: Symbol = symbol_short!("NFT_CTR");
pub const TEMPLATE_COUNTER: Symbol = symbol_short!("TMPL_CTR");
pub const BADGE_COUNTER: Symbol = symbol_short!("BADGE_CTR");
pub const EXPIRED_CREDENTIALS: Symbol = symbol_short!("EXPIRED");
pub const CROSS_CHAIN_REGISTRY: Symbol = symbol_short!("XCHAIN");
