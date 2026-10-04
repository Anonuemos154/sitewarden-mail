# ADR-001 — Desktop-first, browser companion second

## Decision
Build the trusted product as a desktop application, not as a Gmail DOM extension.

## Reasons
- provider-neutral;
- stronger process isolation;
- local vault;
- independent update/signing chain;
- browser extension permissions can remain narrow;
- one security pipeline for Gmail/Microsoft/IMAP.

## Consequences
- more engineering than a browser-only MVP;
- code signing/notarization required;
- OAuth app verification still required;
- browser store listings become acquisition/UX channels rather than the trust root.
