# Implementation status — 0.1.0-alpha foundation

## Implemented in source
- typed domain model;
- provider abstraction;
- deterministic risk-engine seed rules;
- typed SafeDocument reconstruction path;
- reversible cleanup plan model;
- action gateway that blocks automated permanent deletion;
- URL guard with scheme/host/IDN normalization basics;
- MIME size/part/header guard;
- local `.eml` read-only provider foundation;
- Tauri/React desktop shell;
- Manifest V3 browser companion foundation;
- GitHub CI/security/release scaffolding;
- threat model, security invariants, sandbox design and release gates.

## Not production-ready yet
- Gmail OAuth/provider implementation;
- Microsoft Graph provider implementation;
- generic IMAP/SMTP implementation;
- OS credential vault implementation;
- complete MIME body/attachment extraction;
- ClamAV/YARA isolated workers;
- PDF/Office content analysis;
- hardened platform sandboxes;
- signed updater;
- signed production installers;
- provider OAuth verification;
- external security audit.

The public launch may accurately describe this state as an **open-source alpha / active development**.
Do not market it as a finished antivirus or guaranteed security product.
