# Standards provider (IMAP/SMTP)

Goal: support providers outside Google/Microsoft.

Requirements:
- OAuth2 where supported.
- App passwords only where the provider requires them.
- Plain account passwords are a last-resort compatibility mode and must be clearly warned.
- TLS certificate validation cannot be disabled in production.
- STARTTLS downgrade and invalid certificate errors must be hard failures by default.
