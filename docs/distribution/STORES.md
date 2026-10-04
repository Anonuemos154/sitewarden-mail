# Stores and distribution

## Chrome Web Store
- extension, not desktop app;
- developer registration required and Google documents a one-time registration fee;
- privacy/data-use disclosures required;
- minimum permissions;
- current 2026 policy requires collected user data to be necessary to the disclosed single purpose;
- remote executable code is not acceptable under Manifest V3 rules.

## Firefox
- publish/sign through addons.mozilla.org;
- release/beta Firefox requires Mozilla signing even for self-distributed extensions;
- source/build instructions may be required for generated/minified code;
- permissions and data transmission must be narrowly justified.

## Microsoft Edge Add-ons
- Chromium extension can largely reuse the Chrome package;
- Microsoft states there is **no registration fee** for the Edge extension program;
- Partner Center account required.

## Desktop Windows
Initial: signed installer from SiteWarden + GitHub Releases.
Later: winget and Microsoft Store.

## Desktop macOS
Developer ID signing + notarization even when distributed outside the Mac App Store.

## Linux
Signed checksums plus AppImage/Flatpak/deb/rpm as maintenance capacity permits.

## Google Workspace Marketplace
Potential later channel for organization deployment/add-on presence, but not the security root.
Provider OAuth verification is separate from simply publishing a browser extension.
