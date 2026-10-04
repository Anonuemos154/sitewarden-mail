# Security invariants

These are release blockers.

1. **No original HTML in trusted UI.**
2. **No OAuth token in extension storage.**
3. **No provider password in logs.**
4. **No remote code execution in browser extensions.**
5. **No permanent-delete rule.**
6. **No auto-opening attachments.**
7. **No scan worker with provider credentials.**
8. **No attachment parser with unrestricted filesystem access.**
9. **No telemetry containing message bodies/subjects/addresses by default.**
10. **No update without signature verification.**
11. **No unsigned official Windows/macOS build.**
12. **No broad Gmail `https://mail.google.com/` scope merely for convenience.**
13. **No “safe” badge based on one signal alone.**
14. **No hidden network destination.**
15. **No secret security architecture as a required defense.**
16. **No destructive bulk action without preview and explicit confirmation.**
17. **No HTML sanitizer treated as the sole containment boundary.**
18. **No release from a dirty/unreviewed dependency lockfile.**
