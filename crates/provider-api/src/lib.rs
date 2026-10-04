use domain::{AccountId, MessageEnvelope, MessageId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("authentication required")]
    AuthenticationRequired,
    #[error("provider rate limited the request")]
    RateLimited,
    #[error("provider rejected operation: {0}")]
    Rejected(String),
    #[error("network failure")]
    Network,
    #[error("unexpected provider response")]
    Protocol,
}

#[derive(Debug, Clone)]
pub enum MailMutation {
    Archive(Vec<MessageId>),
    Trash(Vec<MessageId>),
    Untrash(Vec<MessageId>),
    MarkRead(Vec<MessageId>, bool),
    AddLabel(Vec<MessageId>, String),
    RemoveLabel(Vec<MessageId>, String),
}

pub trait MailProvider {
    fn account_id(&self) -> &AccountId;
    fn fetch_changed(
        &self,
        cursor: Option<&str>,
    ) -> Result<(Vec<MessageEnvelope>, String), ProviderError>;
    fn apply(&self, mutation: MailMutation) -> Result<(), ProviderError>;
}
