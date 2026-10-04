# Gmail provider

## Target API
Gmail REST API with OAuth 2.0.

## Scope strategy

For a full mail-management client, the likely target is `gmail.modify` rather than the broader
`https://mail.google.com/` scope. `gmail.modify` is still a **restricted scope** and therefore
requires Google's restricted-scope verification for a public app. It allows reading and modifying
mail and does not allow immediate permanent deletion bypassing trash.

Permanent deletion should not be a product requirement.

## Security rule
OAuth refresh tokens stay in the desktop credential vault. The browser extension does not receive
them.

## Verification consequence
If restricted-scope data is stored on or transmitted through project servers, Google states that a
security assessment is required. The architecture therefore keeps mailbox data local by default,
which materially simplifies privacy and risk, but public OAuth verification is still expected.
