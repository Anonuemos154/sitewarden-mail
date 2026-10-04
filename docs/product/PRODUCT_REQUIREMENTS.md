# Product requirements

## Account hub
- Gmail.
- Microsoft 365 / Outlook.com.
- Generic IMAP/SMTP.
- Multiple accounts.
- Unified inbox plus per-account views.
- Local full-text index.
- Account health screen.

## Security hub
- Authentication-result view: SPF/DKIM/DMARC/ARC.
- Sender display-name and Reply-To mismatch checks.
- IDN/Punycode/confusable-domain warnings.
- URL canonicalization and visible-target mismatch.
- Local threat-rule feed.
- Attachment hashes, type detection and quarantine.
- Antivirus worker.
- YARA/static rules worker.
- Archive recursion/zip-bomb controls.
- Office macro and suspicious-container indicators.
- PDF JavaScript/Launch/embedded-file indicators.
- Safe View that never renders original message HTML.
- Tracking-pixel blocking.
- Remote-image blocking by default.
- Explainable risk report: no unexplained “AI says dangerous”.
- Manual false-positive/false-negative reporting without uploading the whole mail by default.

## Inbox operations hub
- Newsletter/promotions grouping.
- Bulk archive/trash with preview.
- Unsubscribe assistance.
- Duplicate / stale notification identification.
- Smart folders.
- Rules.
- “Why was this suggested?” explanation.
- Undo ledger.
- No automatic permanent delete.

## Productivity hub
- Send later.
- Snooze.
- Follow-up reminders.
- Recurring messages.
- Templates.
- Multiple signatures.
- Private notes.
- Saved searches.
- Optional mail sequences after security foundation is mature.
- Optional mail merge after abuse controls are designed.

## Later team hub
- Shared queues.
- Assignment.
- Internal notes.
- Organization policies.
- SIEM/export.
- Admin device health.
- Rule distribution.
- SSO/SCIM for paid managed deployments.

## Explicitly not v1
- Autonomous AI deleting mail.
- Automatic execution/opening of attachments.
- “100% secure” claims.
- Human-readable message content sent to analytics.
- Provider passwords stored in browser localStorage.
