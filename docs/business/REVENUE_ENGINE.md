# Revenue engine

## Objective

Maximize sustainable revenue without weakening the open-source/security positioning.

The revenue model is deliberately **not** "charge everybody €5 for the same local client". The
highest-value customers are organizations that pay for certainty, deployment, integration,
governance, support and licensing.

## Revenue ladder

### 1. Cash now — Alpha stage

#### A. Design Partners
Target price: **€4,900–€9,900 per engagement**.

Sell the engineering relationship, threat-model work, integration planning and prioritized
requirements. Do not sell Alpha as a finished production control.

Target buyers:
- IT/security consultancies;
- MSPs;
- privacy-sensitive SMEs;
- legal/accounting/professional-services firms;
- organizations with Microsoft 365 / Google Workspace exposure;
- security teams interested in local-first tooling.

#### B. Pilot Partners
Target price: **€12,500–€30,000 per 90-day pilot**.

The price should rise with:
- number of mailboxes/users;
- provider complexity;
- on-prem/network constraints;
- custom policy/integration work;
- reporting/support requirements.

#### C. Professional services
Use SiteWarden Mail to create high-margin security work:
- tenant hardening;
- mail architecture review;
- migration;
- incident-readiness;
- secure workflow design;
- integration engineering.

Do not discount these services simply because the core code is open source.

#### D. Sponsorship
Useful for community proof and audit/security funding, but should be treated as additive revenue
rather than the main business model.

### 2. Stable release revenue

Once signed builds and provider integrations are production-ready:

#### Community
€0. Local-first client and core security remain open source.

#### Business
Target range: **€19–€39 per user/month**, preferably annual billing.

Possible paid value:
- managed organization policy;
- central administration;
- fleet/device health;
- managed threat/rule feed;
- team workflows;
- longer support/retention;
- organization analytics that do not require raw mail content;
- managed deployment.

A price in this range is commercially defensible because current email-productivity tools already
charge roughly €8–€21/user/month and premium productivity clients around $25–$33/user/month before
adding a security-management layer.

#### Enterprise
Custom annual contract with minimum commitment.

Sell:
- SSO/SCIM;
- SIEM;
- policy/fleet control;
- deployment architecture;
- dedicated support;
- contractual response targets;
- compliance/security documentation;
- custom integration.

Avoid a low public enterprise ceiling. Use a minimum contract value instead.

Planning target after product maturity:
**€25k–€100k+ ARR per enterprise account**, depending on scope.

### 3. High-margin licensing

#### Commercial AGPL exception / OEM
For vendors that need to embed, redistribute or keep modifications proprietary.

Planning floor:
**€25k/year**, with higher pricing for broad redistribution, white-label use, appliances, hosted
services or strategic support.

Never grant a perpetual unrestricted commercial license cheaply in the early project stage.

#### Sponsored development
A company can fund a feature while the resulting code remains open.

Typical range:
**€5k–€50k+ per milestone**, depending on engineering/security review burden.

### 4. Non-dilutive funding

Priority current opportunities (verify eligibility before submission):
- NGI / Open Internet Stack calls;
- Prototype Fund Germany;
- GitHub Secure Open Source Fund;
- security/privacy research and open-source grants.

Grant money should finance public-good engineering such as sandboxing, reproducible builds,
auditing and interoperability, while commercial revenue pays for deployment/support/enterprise
features.

## Sales funnel

1. Free Community alpha creates credibility and technical evidence.
2. Website CTA: "Become a founding design partner".
3. Qualification call.
4. Paid discovery/design engagement.
5. 90-day paid pilot.
6. Annual Enterprise / managed support contract.
7. Optional commercial/OEM license.

Do not skip directly from free download to a €10/month consumer subscription if the buyer is an
organization with a five-figure security problem.

## Public pricing strategy

Publish:
- Community: Free.
- Design Partner: "from €4,900".
- Pilot Partner: "from €12,500".
- Enterprise/OEM: "Contact / custom".

Keep exact enterprise pricing private so scope and willingness-to-pay can be captured in the quote.

## Commercial guardrails

- Never claim that Alpha is production-safe.
- Never sell "100% protection".
- Use a written statement of work for paid pilots/services.
- Separate sponsorship from contractual support.
- Track contributor copyright so dual licensing remains possible.
- Require contributor licensing/assignment strategy before accepting large external code
  contributions if future dual licensing is important.
