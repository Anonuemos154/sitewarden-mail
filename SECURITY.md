# Security policy

## Reporting a vulnerability

Do not open a public issue for a vulnerability that could expose mailbox data, credentials,
arbitrary code execution or updater compromise.

Before launch, publish a dedicated address such as:

`security@sitewarden.de`

and a `security.txt` file on the project website.

## Security principles

1. Assume the source code is public and fully understood by attackers.
2. Keep credentials out of browser content scripts and renderer/UI code.
3. Treat email, attachments, URLs, MIME parsers and antivirus engines as hostile input.
4. Prefer explicit allow-lists and typed data models over sanitizing arbitrary HTML.
5. Make destructive actions reversible.
6. No remote executable code in extensions.
7. Pin dependencies for releases.
8. Produce an SBOM for each official release.
9. Sign release artifacts.
10. Publish supported-version and vulnerability-response policies.

See `docs/security/`.
