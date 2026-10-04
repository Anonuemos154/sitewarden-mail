# Implementation baseline — 2026-10-04

SiteWarden Mail is an **Alpha / Active Development** security tool. This baseline
records what was verifiably available at the public-launch checkpoint.

## Verified locally

- The React/TypeScript desktop frontend compiles with `pnpm run build`.
- Dependencies are locked in `apps/desktop/pnpm-lock.yaml`.
- Only `esbuild` is permitted to execute an install script through
  `apps/desktop/pnpm-workspace.yaml`.
- Safe View presents plain text and neutralized link information; it does not
  render untrusted message HTML.
- The local EML provider is read-only.

## Not verified on this workstation

- Rust formatting, Clippy, workspace tests, and a native Tauri package were not
  executed because a Rust toolchain is not installed here. GitHub CI defines
  these checks and must pass before any signed or stable release.
- Gmail, Microsoft 365, and generic IMAP integrations are roadmap work and must
  not be described as production-ready.
- Full antivirus coverage, attachment sandboxing, signed installers, and store
  approval are not present.

## Release posture

No stable binary or download is offered. The source may be published only after
the owner confirms the final license and the repository's required checks are
green. Until all release gates pass, every public surface must retain the
**Alpha / Active Development** label.
