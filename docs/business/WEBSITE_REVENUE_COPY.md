# MSD website revenue / product copy — no public prices

## Site structure

MSD International is the flagship. SiteWarden is the security line and SiteWarden Mail is a product
within the Tools / Systems portfolio.

### Navigation
- Systems
- Security
- Tools
- Company
- Contact

**Do not expose an Admin link in header, footer, mobile navigation, sitemap, structured data or
public page content.** Administrative access must use a known direct route plus authentication;
security must not depend on obscurity.

## Homepage hero
**Systems for complex environments.**

MSD develops and operates digital systems where software, data, automation and security meet
real-world operations.

Primary CTA: **Explore systems**
Secondary CTA: **View tools**

## Tools page — SiteWarden Mail card
### SiteWarden Mail
**Open, local-first email security and inbox operations.**

A desktop mail-security platform under active development. SiteWarden Mail is designed to reduce
trust in active email content, reconstruct messages into safer representations and combine security
signals with inbox management.

CTAs:
- **Download Alpha**
- **View on GitHub**
- **Become a design partner**

Download target:
https://github.com/Anonuemos154/sitewarden-mail/releases/tag/alpha-latest

GitHub:
https://github.com/Anonuemos154/sitewarden-mail

## Product page hero
**SiteWarden Mail**
### Treat email as untrusted input.

Local-first email security, safe reconstruction and inbox operations in one open-source desktop
platform.

Badges:
- Open Source
- AGPL-3.0-or-later
- Local-first
- Alpha / Active Development

Primary CTA: **Download Windows Alpha**
Secondary CTA: **View source**
Tertiary CTA: **Design partner / enterprise**

## Download notice
**Alpha software.** The current public build is for evaluation and UI/security-architecture testing.
Provider integrations and production-grade attachment isolation are still under development. The
installer is currently unsigned unless the release page explicitly states otherwise.

## Commercial section
### Build with us
Organizations can engage as design partners, pilot partners, enterprise users, strategic sponsors
or OEM/commercial-license partners. Scope and commercial terms are agreed individually.

CTA: **Discuss a deployment**

## Privacy/contact rules
- Never publish a private personal Gmail/Googlemail address.
- Use role-based business contact routes only.
- Do not embed private addresses in HTML comments, JavaScript, JSON-LD, source maps, environment
  templates, repository metadata or downloadable example configs.
- Public business contact data must follow applicable German legal requirements.

## Admin removal acceptance test
A deploy is not complete until all of the following return no visible Admin navigation:
- desktop header/footer;
- mobile menu/footer;
- Tools page;
- SiteWarden Mail product page;
- sitemap and generated navigation data.

The admin route itself must be noindex and protected by real authentication.
