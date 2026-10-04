use domain::MessageId;
use provider_api::{MailMutation, MailProvider, ProviderError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestedAction {
    Archive(Vec<MessageId>),
    Trash(Vec<MessageId>),
    MarkRead(Vec<MessageId>, bool),
    PermanentDelete(Vec<MessageId>),
}

#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("permanent deletion is not permitted through automation")]
    PermanentDeleteBlocked,
    #[error(transparent)]
    Provider(#[from] ProviderError),
}

pub fn execute_confirmed<P: MailProvider>(
    provider: &P,
    action: RequestedAction,
) -> Result<(), ActionError> {
    let mutation = match action {
        RequestedAction::Archive(ids) => MailMutation::Archive(ids),
        RequestedAction::Trash(ids) => MailMutation::Trash(ids),
        RequestedAction::MarkRead(ids, value) => MailMutation::MarkRead(ids, value),
        RequestedAction::PermanentDelete(_) => return Err(ActionError::PermanentDeleteBlocked),
    };
    provider.apply(mutation)?;
    Ok(())
}
