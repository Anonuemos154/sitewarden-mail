# Provider permissions

## Gmail
For a full mailbox client, `gmail.modify` is a likely target:
- read mail;
- compose/send;
- modify mailbox state;
- cannot immediately permanently delete bypassing trash.

It is classified by Google as a **restricted scope**.

Avoid the broader `https://mail.google.com/` scope unless immediate permanent deletion is truly
required. This product should not require it.

Google states that storing or transmitting restricted-scope data on project servers triggers a
security assessment requirement. Keeping message content local is therefore strategically valuable.

Official reference:
https://developers.google.com/workspace/gmail/api/auth/scopes

## Microsoft
Use delegated permissions:
- `Mail.ReadWrite` for read/update/delete-to-trash style mail operations as needed;
- `Mail.Send` separately for sending.

Avoid application permissions that allow access to every mailbox for the consumer client.

Official reference:
https://learn.microsoft.com/en-us/graph/permissions-reference
