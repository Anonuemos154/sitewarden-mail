# SiteWarden Mail

> **Alpha / Active Development.** No stable or signed binary is available yet.

SiteWarden Mail is planned as a **local-first, open-source email security and productivity client**:
one desktop application for Gmail, Microsoft 365/Outlook and standards-based mailboxes, with a
separate browser companion.

This repository is intentionally a **foundation**, not a fake “finished antivirus”. It defines the
architecture, security boundaries, provider strategy, product scope, funding model and release path
before a public beta is shipped.

## Product promise

The application should combine four product classes that are usually separate:

1. **Unified email client** — multiple accounts, folders, search, compose, scheduling.
2. **Security gateway on the endpoint** — sender/authentication analysis, URL inspection,
   attachment scanning, inert Safe View, quarantine and explainable risk signals.
3. **Inbox operations** — cleanup, bulk archive/trash, unsubscribe assistance, duplicate/noise
   reduction, rules and reversible automation.
4. **Productivity layer** — snooze, reminders, recurring mail, templates, notes, follow-up
   workflows and later team features.

## Non-negotiable architecture

- Email HTML is **never rendered directly in the trusted UI**.
- OAuth passwords/tokens are **never stored in the browser extension**.
- Raw messages and attachments are processed in **separate workers** with the smallest possible
  privileges.
- No telemetry by default.
- No cloud upload of raw mailbox content by default.
- Destructive actions are reversible by default.
- Permanent delete is never a background automation.
- Security does not depend on source-code secrecy.
- Official release artifacts must be signed and reproducible as far as practical.
- The product must never advertise “100% secure”.

## Target distribution

- Windows: signed installer / Microsoft Store later.
- macOS: Developer ID signed + notarized package; Mac App Store later if useful.
- Linux: AppImage/Flatpak/deb/rpm as project maturity allows.
- Chrome Web Store: companion extension.
- Firefox AMO: companion extension.
- Microsoft Edge Add-ons: companion extension.
- GitHub Releases: source + checksums + SBOM + provenance.

## Repository map

- `docs/product/` — product scope, competitors, roadmap.
- `docs/security/` — threat model, invariants, sandbox and release security.
- `docs/business/` — monetization, funding, launch and sponsorship.
- `docs/legal/` — non-final legal/compliance notes and draft clauses.
- `docs/distribution/` — stores and official release path.
- `crates/` — planned Rust security core.
- `providers/` — Gmail, Microsoft Graph and IMAP adapters.
- `apps/desktop/` — desktop shell.
- `apps/browser-extension/` — thin browser companion only.

Read `IMPLEMENTATION_STATUS.md` before treating anything here as production-ready.

## License

SiteWarden Mail is licensed under `AGPL-3.0-or-later`. See `LICENSE` and
`LICENSE-STRATEGY.md`. SiteWarden and MSD names and logos are not granted as
trademarks by the software license.


## Commercial partnerships and sponsorship

The Community edition is open source and intended to remain free. Organizations can fund or adopt
the project through:

- **Founding sponsorships**
- **Design Partner engagements** from €4,900
- **Pilot Partner engagements** from €12,500
- **Enterprise / OEM / commercial licensing**
- **Professional deployment and security services**

During Alpha, paid engagements are evaluation, co-development or consulting work rather than a
production security SLA.

See [COMMERCIAL.md](COMMERCIAL.md) for commercial options and
[SPONSORING.md](SPONSORING.md) for project sponsorship.
