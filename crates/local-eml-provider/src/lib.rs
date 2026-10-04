use domain::*;
use mailparse::MailHeaderMap;
use provider_api::{MailMutation, MailProvider, ProviderError};
use std::{
    fs,
    path::{Path, PathBuf},
};
use time::OffsetDateTime;

pub struct LocalEmlProvider {
    root: PathBuf,
    account: AccountId,
}

impl LocalEmlProvider {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            account: AccountId("local-eml".into()),
        }
    }

    fn parse_one(&self, path: &Path) -> Result<MessageEnvelope, ProviderError> {
        let raw = fs::read(path).map_err(|_| ProviderError::Protocol)?;
        let parsed = mailparse::parse_mail(&raw).map_err(|_| ProviderError::Protocol)?;
        let headers = parsed.get_headers();
        let subject = headers.get_first_value("Subject").unwrap_or_default();
        let from_raw = headers.get_first_value("From").unwrap_or_default();
        let reply_raw = headers.get_first_value("Reply-To");
        let body = parsed.get_body().ok();
        Ok(MessageEnvelope {
            account_id: self.account.clone(),
            message_id: MessageId(
                path.file_name()
                    .and_then(|x| x.to_str())
                    .unwrap_or("mail.eml")
                    .into(),
            ),
            thread_id: None,
            from: MailAddress {
                display_name: None,
                address: from_raw,
            },
            reply_to: reply_raw.map(|x| MailAddress {
                display_name: None,
                address: x,
            }),
            to: vec![],
            cc: vec![],
            subject,
            received_at: OffsetDateTime::now_utc(),
            auth: HeaderAuth {
                spf: AuthResult::None,
                dkim: AuthResult::None,
                dmarc: AuthResult::None,
                arc: AuthResult::None,
            },
            plain_text: body,
            raw_html: None,
            links: vec![],
            attachments: vec![],
        })
    }
}

impl MailProvider for LocalEmlProvider {
    fn account_id(&self) -> &AccountId {
        &self.account
    }
    fn fetch_changed(
        &self,
        _cursor: Option<&str>,
    ) -> Result<(Vec<MessageEnvelope>, String), ProviderError> {
        let mut out = vec![];
        let entries = fs::read_dir(&self.root).map_err(|_| ProviderError::Protocol)?;
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension()
                .and_then(|x| x.to_str())
                .map(|x| x.eq_ignore_ascii_case("eml"))
                .unwrap_or(false)
            {
                if let Ok(m) = self.parse_one(&p) {
                    out.push(m);
                }
            }
        }
        Ok((out, "local-scan".into()))
    }
    fn apply(&self, _mutation: MailMutation) -> Result<(), ProviderError> {
        Err(ProviderError::Rejected(
            "local EML provider is read-only".into(),
        ))
    }
}
