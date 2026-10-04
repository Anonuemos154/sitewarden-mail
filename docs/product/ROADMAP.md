# Roadmap

## Phase 0 — Foundation
Goal: public architecture and credible project.

- Threat model.
- License decision.
- Brand/name clearance.
- Repository governance.
- Prototype Fund / NGI applications.
- UI prototype with no real mailbox access.
- Provider permission design.

Exit: architecture review complete.

## Phase 1 — Private alpha
Goal: one real mailbox provider, read-only first.

- Gmail OAuth verification preparation.
- Read-only sync.
- Local encrypted index.
- MIME parser worker.
- header/domain/link analysis.
- Safe View.
- test mailbox corpus.
- no send / no delete.

Exit: 100k-message corpus fuzz/regression suite and stable local data model.

## Phase 2 — Controlled actions
- `gmail.modify`.
- archive/trash/untrash.
- cleanup plans.
- Microsoft Graph.
- send/reply.
- signed rule-feed prototype.
- attachment quarantine and ClamAV/YARA workers.

Exit: independent architecture/security review.

## Phase 3 — Public beta
- signed Windows installer.
- notarized macOS build.
- Linux package.
- Chrome/Firefox/Edge companion.
- public vulnerability policy.
- SBOM and signed checksums.
- optional supporter/sponsor program.

## Phase 4 — Security hardening
- OS-specific sandboxes.
- content disarm previews.
- local domain reputation database.
- secure auto-update.
- reproducible-build work.
- external penetration test.
- bug bounty.

## Phase 5 — Business/enterprise
- organization policy console.
- fleet health.
- SSO/SCIM.
- SIEM integration.
- managed rule feed.
- paid support/SLA.
- SiteWarden assessments and deployment projects.
