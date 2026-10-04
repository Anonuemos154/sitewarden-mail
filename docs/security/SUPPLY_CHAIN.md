# Supply-chain and release security

Required before public stable release:

- locked dependencies;
- automated vulnerability checks;
- license inventory;
- SBOM (SPDX or CycloneDX);
- signed release checksums;
- Windows code signing;
- Apple Developer ID + notarization;
- provenance/attestation;
- protected release branch;
- mandatory review for security-sensitive files;
- hardware-backed or hosted hardened signing key;
- reproducible-build work where practical;
- release runbook and revocation procedure.

Official builds should be clearly distinguishable from community forks through cryptographic
signing and trademark policy, not through closed source.
