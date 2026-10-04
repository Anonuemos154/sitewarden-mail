# Contributing

Security-sensitive changes require:

- a threat-model note;
- tests for malformed/untrusted input;
- a review by at least one maintainer other than the author before release;
- no new broad permission without documentation;
- no new network destination without documentation;
- no use of `innerHTML`/equivalent for message content in trusted UI.

Large changes should begin with a design issue describing trust-boundary impact.
