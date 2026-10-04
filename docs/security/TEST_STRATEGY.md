# Security test strategy

## Unit
- URL canonicalization.
- IDN/confusable edge cases.
- risk-score determinism.
- action-preview/undo logic.
- parser limits.
- SafeDocument cannot carry raw HTML.

## Corpus/regression
Maintain samples for:
- malformed MIME;
- nested multipart;
- huge headers;
- zip bombs;
- password-protected archives;
- macro documents;
- PDF active content;
- disguised executable extensions;
- Unicode sender/domain spoofs;
- QR-code links;
- thread-hijack patterns.

## Fuzzing
Fuzz parsers, canonicalizers, rule engine and native bridge protocol.

## Integration
Use dedicated test accounts, never developer personal inboxes.

## Adversarial review
Before stable:
- independent code review;
- update-chain review;
- OAuth/token storage review;
- extension-permission review;
- sandbox-escape-oriented testing.
