# Privacy architecture

## Default
- account connects directly from desktop to mail provider;
- project server does not receive message bodies;
- telemetry off;
- remote image loading off;
- crash reports opt-in and scrubbed;
- threat intelligence preferably downloaded to client rather than user URLs uploaded to server.

## Data classes
1. OAuth credential.
2. raw email.
3. attachment.
4. contact/address metadata.
5. local rules.
6. risk report.
7. optional diagnostic bundle.

Each class needs:
- purpose;
- storage location;
- encryption;
- retention;
- export/delete behavior;
- whether it ever leaves the device.

## Enterprise cloud
If introduced, design the admin console so it can operate on device/security state without central
mail content wherever possible.
