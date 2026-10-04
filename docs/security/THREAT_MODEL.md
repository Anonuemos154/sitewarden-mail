# Threat model

## Assets
- OAuth refresh/access tokens.
- mailbox content and attachments.
- address book and correspondence graph.
- local search index.
- rules and organization policy.
- update-signing chain.
- release signing keys.

## Primary adversaries
1. malicious sender with arbitrary MIME/HTML/attachment input;
2. phishing operator exploiting human trust;
3. compromised legitimate sender account;
4. malicious/compromised website linked from a message;
5. malicious attachment exploiting a parser/AV engine;
6. compromised dependency/update infrastructure;
7. local unprivileged malware;
8. malicious browser extension;
9. malicious contributor/supply-chain change.

## Key boundaries
- provider network -> provider worker;
- provider worker -> canonical envelope;
- raw MIME -> parser worker;
- attachment -> scanner sandbox;
- raw content -> safe-render worker;
- safe document -> UI;
- UI intent -> action gateway;
- release CI -> signing identity.

## Security objectives
- provider credentials unavailable to message renderers/scanners;
- raw HTML unavailable to trusted UI;
- scanner crashes do not compromise the desktop process;
- a message cannot trigger mailbox actions;
- cleanup automation cannot permanently delete;
- a malicious update cannot be installed without signature validation;
- logs do not contain raw messages unless explicitly exported.

## Important residual risks
A desktop application cannot truthfully guarantee safety if the underlying OS/kernel or provider
account is fully compromised. The project should minimize blast radius and make attacks harder, but
must not market impossibility claims.
