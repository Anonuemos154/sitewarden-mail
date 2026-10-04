# SiteWarden Mail Alpha — user guide

## What you are downloading

The current Windows Alpha is an **evaluation build** of the SiteWarden Mail desktop interface and
security architecture.

It is not yet a production mail client.

### Working in the current public Alpha
- desktop application shell;
- unified-inbox UI using synthetic demo messages;
- message/Safe View presentation;
- deterministic risk indicators;
- Security Center views;
- quarantine/cleanup/rules/productivity screens;
- local security-core libraries and tests.

### Not yet production-ready
- Gmail OAuth mailbox connection;
- Microsoft Graph mailbox connection;
- generic IMAP/SMTP account setup;
- signed Windows publisher certificate;
- hardened attachment antivirus/sandbox worker;
- production updater;
- external security audit.

## Install on Windows

1. Open the latest Alpha release:
   https://github.com/Anonuemos154/sitewarden-mail/releases/tag/alpha-latest
2. Download the Windows setup executable (or MSI if present).
3. Check the release notes and SHA/checksum information.
4. Because the Alpha may currently be unsigned, Windows can display a publisher warning. Do not
   bypass such warnings unless you intentionally downloaded the file from the official repository
   above and understand that this is an evaluation build.
5. Complete installation and start **SiteWarden Mail** from the Start menu.

## First test

The current application intentionally opens with synthetic messages. This lets you inspect the
interface without granting access to a real mailbox.

Try:
1. Inbox — select several synthetic messages.
2. Safe View — inspect how message content is represented.
3. Security — inspect risk signals and explanations.
4. Quarantine — review the intended isolation workflow.
5. Cleanup — inspect reversible cleanup concepts.
6. Rules/Productivity/Accounts — review the planned management surfaces.

## Important

Do not use the current Alpha as your only protection for a production mailbox. Provider connection
and attachment-isolation work is still under active development.

The next major usability milestone is real provider connection through Gmail, Microsoft Graph and
standards-based IMAP/SMTP.
