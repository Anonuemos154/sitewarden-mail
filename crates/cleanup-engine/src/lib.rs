use domain::MessageId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlannedAction {
    Archive(MessageId),
    Trash(MessageId),
    AddLabel(MessageId, String),
    RemoveLabel(MessageId, String),
    MarkRead(MessageId, bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPlan {
    pub plan_id: Uuid,
    pub reason: String,
    pub actions: Vec<PlannedAction>,
    pub requires_confirmation: bool,
}

/// Design invariant:
/// This engine creates *plans*. It does not directly mutate provider state.
pub fn plan_noise_cleanup(message_ids: Vec<MessageId>, reason: impl Into<String>) -> CleanupPlan {
    CleanupPlan {
        plan_id: Uuid::new_v4(),
        reason: reason.into(),
        actions: message_ids
            .into_iter()
            .map(PlannedAction::Archive)
            .collect(),
        requires_confirmation: true,
    }
}
