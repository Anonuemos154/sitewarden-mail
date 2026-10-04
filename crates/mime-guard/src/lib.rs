use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimeLimits {
    pub max_message_bytes: usize,
    pub max_parts: usize,
    pub max_header_bytes: usize,
}
impl Default for MimeLimits {
    fn default() -> Self {
        Self {
            max_message_bytes: 50 * 1024 * 1024,
            max_parts: 500,
            max_header_bytes: 256 * 1024,
        }
    }
}

#[derive(Debug, Error)]
pub enum GuardError {
    #[error("message exceeds size limit")]
    MessageTooLarge,
    #[error("too many MIME parts")]
    TooManyParts,
    #[error("headers exceed limit")]
    HeadersTooLarge,
    #[error("MIME parse failed")]
    Parse,
}

pub fn validate(raw: &[u8], limits: &MimeLimits) -> Result<(), GuardError> {
    if raw.len() > limits.max_message_bytes {
        return Err(GuardError::MessageTooLarge);
    }
    let header_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .or_else(|| raw.windows(2).position(|w| w == b"\n\n"))
        .unwrap_or(raw.len());
    if header_end > limits.max_header_bytes {
        return Err(GuardError::HeadersTooLarge);
    }
    let parsed = mailparse::parse_mail(raw).map_err(|_| GuardError::Parse)?;
    fn count_parts(p: &mailparse::ParsedMail<'_>) -> usize {
        1 + p.subparts.iter().map(count_parts).sum::<usize>()
    }
    if count_parts(&parsed) > limits.max_parts {
        return Err(GuardError::TooManyParts);
    }
    Ok(())
}
