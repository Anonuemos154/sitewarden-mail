# Architecture

## 1. Trust zones

```text
                     PROVIDERS / INTERNET
       Gmail API        Microsoft Graph        IMAP/SMTP
           |                  |                    |
           +------------------+--------------------+
                              |
                     [ Provider Workers ]
                              |
                    canonical MessageEnvelope
                              |
                              v
                    [ Message Intake Bus ]
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
      [Header/Auth]     [URL Analyzer]   [Attachment Worker]
             |                |                |
             |                |        AV / YARA / static
             |                |        analysis / archive limits
             +----------------+----------------+
                              |
                              v
                       [ Risk Engine ]
                              |
             +----------------+----------------+
             |                                 |
             v                                 v
       [Quarantine]                      [Safe Renderer]
                                               |
                                      inert SafeDocument
                                               |
                                               v
                                      [ Trusted Desktop UI ]
                                               |
                          +--------------------+--------------------+
                          |                    |                    |
                          v                    v                    v
                      Read/Reply          Cleanup Plan         Search/Rules
                          |                    |
                          v                    v
                    [Action Gateway] -- reversible provider actions
```

## 2. Desktop app

Target: **Tauri 2 shell + Rust core**.

The Tauri frontend is not allowed to render arbitrary email HTML. It receives only trusted domain
objects such as `SafeDocument`, `RiskReport`, `MailboxItem`, `CleanupPlan` and `ActionReceipt`.

Security-sensitive parsing and scanning must live outside the UI process whenever feasible.

## 3. Worker processes

Planned worker types:

### Provider worker
Network access only to the configured provider endpoints. Produces canonical messages. Does not
render content.

### Parser worker
Parses MIME with strict size, nesting and recursion limits. No provider credentials. No general
network access.

### Attachment worker
Scans extracted attachments in a sandbox. AV engines and native parsers are treated as untrusted
dependencies and isolated from OAuth tokens and the UI.

### Safe-render worker
Converts allowed content into `SafeDocument`. The trusted UI never receives original message HTML.

## 4. Storage

Local-first.

- SQLite or equivalent for index/metadata.
- Message bodies encrypted at rest at the application layer.
- Master key derived/stored via OS secure storage:
  - Windows: DPAPI / Credential Locker.
  - macOS: Keychain.
  - Linux: Secret Service / keyring when available.
- OAuth refresh tokens stored separately from message cache.
- Attachments stored only when explicitly cached or quarantined.
- Cache can be configured to metadata-only.

## 5. Cloud

Core functionality must not require a SiteWarden cloud.

Optional services may exist later for:

- signed threat/rule feeds;
- organization policy distribution;
- device inventory;
- enterprise audit/health status;
- support diagnostics explicitly exported by the user.

Raw mailbox content must not be uploaded by default.

## 6. Browser extension

The extension is a **companion**, not the security root.

Allowed roles:

- open the current thread in the desktop app;
- show a local risk badge supplied by the desktop app;
- hand a user-selected message reference to the desktop app;
- provide a one-click Safe View entry point.

It should not retain OAuth tokens or become a second email client.

## 7. Action Gateway

All mailbox-changing actions pass through one gateway:

- label/move/archive;
- trash/untrash;
- mark read/unread;
- send/schedule;
- unsubscribe helper;
- bulk cleanup.

Every destructive batch operation first creates a preview (`CleanupPlan`) and, where the provider
allows it, an undo receipt. Permanent deletion is outside automatic rules.
