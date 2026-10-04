# Browser companion

This is intentionally thin. It is not the mailbox engine.

## Chrome / Edge
Use Manifest V3 and native messaging to the installed desktop bridge. The native-host manifest
uses `allowed_origins`.

## Firefox
Maintain a Firefox-specific manifest/build output. Firefox native messaging uses a separate host
manifest shape (`allowed_extensions`). Do not blindly submit the Chromium host manifest.

## Permissions
`activeTab`, `contextMenus` and `nativeMessaging` must each be justified in store disclosures.
Avoid broad host permissions unless a later feature cannot work without them.
