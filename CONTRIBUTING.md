# Contributing

Security-sensitive changes require:

- a threat-model note;
- tests for malformed/untrusted input;
- a review by at least one maintainer other than the author before release;
- no new broad permission without documentation;
- no new network destination without documentation;
- no use of `innerHTML`/equivalent for message content in trusted UI.

Large changes should begin with a design issue describing trust-boundary impact.


## Contributor rights during the Alpha licensing phase

The project intends to preserve the option of offering separate commercial licenses in addition to
AGPL-3.0-or-later.

Until a lawyer-reviewed contributor licensing process is published, maintainers may hold or decline
substantial third-party code contributions that would make future dual licensing impractical.
Issues, testing, design feedback and small documentation corrections are still welcome.

Do not interpret this section as a copyright assignment or CLA. No copyright assignment is created
by this file alone. A formal contributor agreement, if adopted, will be published separately and
must be explicitly accepted through the defined contribution process.
