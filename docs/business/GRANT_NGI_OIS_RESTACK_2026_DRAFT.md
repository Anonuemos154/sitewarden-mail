# NGI OIS Restack 2026 — application draft

> Draft prepared for the call closing **3 November 2026, 12:00 CET**.
> Current call information should be re-checked immediately before submission.

## Working title

**SiteWarden Mail — Open, Local-First Email Security and Operations**

## One-sentence summary

SiteWarden Mail is an open-source, local-first email client and security layer that reduces reliance
on active email content by analysing messages, reconstructing them into safer representations and
combining security with mailbox operations across Gmail, Microsoft 365 and standards-based mail.

## Problem

Email remains one of the most trusted interfaces in daily digital life while modern attacks
increasingly abuse legitimate accounts, existing threads, links, attachments and social trust.

Existing products split the problem across:
- proprietary mail clients;
- cloud email-security gateways;
- cleanup tools;
- browser productivity extensions;
- endpoint antivirus.

Users and smaller organisations therefore face a trade-off between convenience, privacy,
interoperability and security visibility.

## Proposed open solution

SiteWarden Mail aims to provide one inspectable open stack:

1. provider-neutral mailbox access;
2. local-first encrypted storage and search;
3. deterministic sender/domain/link risk signals;
4. isolated attachment-analysis workers;
5. Safe View that does not feed original active email HTML directly into the trusted UI;
6. reversible inbox cleanup and operations;
7. productivity features on the same local trust boundary;
8. documented threat model and security invariants.

The project does not claim perfect detection. Its research/engineering goal is to reduce the amount
of authority given to untrusted message content and make risk decisions more inspectable.

## Why this fits Open Internet Stack / digital commons

The project is intended as reusable open infrastructure rather than a proprietary mail silo:

- source code under AGPL-3.0-or-later;
- interoperable provider adapters;
- local-first processing;
- open message/risk data structures;
- documented security architecture;
- user autonomy over mailbox data;
- no mandatory project cloud for core functionality;
- reusable components for safe rendering, link analysis and reversible mailbox actions.

## Requested project scope

### Work package 1 — provider interoperability
- Gmail adapter;
- Microsoft Graph adapter;
- IMAP/SMTP compatibility layer;
- canonical provider-neutral message model.

### Work package 2 — safe content pipeline
- strict MIME parser boundary;
- inert SafeDocument format;
- isolated render/reconstruction path;
- remote-resource blocking;
- URL canonicalisation and IDN/confusable handling.

### Work package 3 — attachment security
- worker-process sandbox;
- file-type validation;
- archive recursion limits;
- AV/YARA adapter architecture;
- Office/PDF active-content indicators.

### Work package 4 — reproducible security
- fuzzing corpus;
- CI/CodeQL;
- dependency pinning;
- SBOM;
- signed release process;
- public security documentation.

### Work package 5 — usable open alpha
- desktop client;
- multi-account inbox;
- explainable risk UI;
- cleanup plan/undo;
- public documentation and contributor process.

## Public outputs

- source repository;
- architecture and threat model;
- provider adapters;
- risk and SafeDocument schemas;
- test fixtures that contain no real user mail;
- release/build documentation;
- SBOM and security-release process;
- public project report.

## Privacy approach

Core mailbox content is processed locally by default.

The project architecture intentionally avoids requiring raw user mailbox content to be uploaded to
a SiteWarden/MSD cloud service.

Optional future managed services should operate on policy/device/security state without raw mail
content wherever practical.

## Security approach

Security is based on documented boundaries rather than secret implementation details.

Examples:
- raw HTML is not accepted as trusted UI input;
- provider credentials are separated from rendering/scanning workers;
- destructive mailbox actions pass through a dedicated action gateway;
- permanent deletion is not an automation target;
- update/release integrity is treated as part of the threat model.

## Sustainability

The open Community client is intended to remain available without charge.

Long-term sustainability comes from:
- enterprise deployment/support;
- managed organisation policy;
- commercial/OEM licensing;
- sponsored development;
- professional security/integration services;
- voluntary sponsorship.

This allows the public technical core to remain open while organisations fund operational value.

## Current state

At application time:
- public AGPL repository exists;
- public Alpha product pages exist;
- CI and CodeQL are enabled;
- desktop/web foundation exists;
- safe-render/risk/cleanup architecture exists;
- production provider verification, attachment isolation and signed stable binaries remain planned
  work.

## Funding range

A sensible request should match the exact work packages selected.

Suggested framing:
- **€25k–€50k** if the proposal includes provider interoperability, safe rendering, sandboxing,
  testing and release hardening;
- narrower request if the call expects one focused R&D milestone.

Do not inflate the request beyond deliverable engineering capacity.

## Evaluation message

The strongest argument is not "better spam filtering".

It is:

> **Reduce trust in active email content while preserving an open, interoperable and user-controlled
> communications stack.**

## Submission checklist

- [ ] verify exact OIS Restack application form fields;
- [ ] select legal applicant;
- [ ] confirm requested amount and eligible costs;
- [ ] convert this draft into form answers;
- [ ] add maintainer biographies;
- [ ] add repository/product URLs;
- [ ] add six-month milestone table;
- [ ] add budget table;
- [ ] review IP/open-source commitments;
- [ ] submit before the call deadline;
- [ ] save final PDF/export and confirmation.
