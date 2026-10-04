use domain::{AccountId, MessageEnvelope, MessageId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("key unavailable")]
    KeyUnavailable,
    #[error("encryption/decryption failure")]
    Crypto,
    #[error("storage failure")]
    Storage,
}

/// Interface only. Production implementation must use authenticated encryption
/// and OS-backed key storage. Do not write OAuth refresh tokens into ordinary SQLite fields.
pub trait Vault {
    fn put_message(&self, message: &MessageEnvelope) -> Result<(), VaultError>;
    fn get_message(&self, account: &AccountId, id: &MessageId) -> Result<Option<MessageEnvelope>, VaultError>;
    fn delete_cached_message(&self, account: &AccountId, id: &MessageId) -> Result<(), VaultError>;
}
