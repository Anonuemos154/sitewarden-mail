# Microsoft Graph provider

Use delegated permissions for an interactive end-user client.

Likely capabilities:
- `Mail.ReadWrite` for mailbox read/write.
- `Mail.Send` only when the user enables sending.

Avoid application-wide mailbox permissions for the consumer desktop product. Enterprise deployment
may have separate admin-consent architecture later.
