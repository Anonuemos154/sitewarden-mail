# Scanner pipeline

## Stage A — Envelope/header
Inputs: provider metadata + raw headers.

Signals:
- SPF/DKIM/DMARC/ARC results supplied by the receiving provider.
- From vs Reply-To.
- display-name impersonation patterns.
- Return-Path mismatch context.
- Message-ID anomalies.
- thread participant change.

Do not recompute SPF from the desktop client; the receiving mail infrastructure has the relevant
SMTP context.

## Stage B — URL
- normalize scheme/host/port/path;
- decode IDN to ASCII and display both Unicode + punycode when suspicious;
- Unicode confusable analysis;
- visible text vs actual target;
- IP-literal URLs;
- unusual ports;
- redirect URLs;
- local signed reputation dataset;
- remote network follow/check only in a separate, opt-in sandbox because it can disclose interest
  in the URL.

## Stage C — Attachment
Before parsing:
- size limit;
- extension vs magic/type mismatch;
- SHA-256;
- archive nesting/ratio limits;
- executable/script list;
- encrypted archive policy.

Worker checks:
- antivirus;
- YARA/static indicators;
- Office macro/container metadata;
- PDF active-content indicators;
- embedded files;
- suspicious shortcut/script containers.

Never execute attachment content.

## Stage D — Safe reconstruction
- plain text preferred;
- HTML converted outside trusted UI;
- no script/event/style execution;
- links represented as typed data;
- remote images not fetched;
- images that are shown should be locally decoded/re-encoded by an isolated worker;
- future “pixel mode” can render in a disposable worker and export only normalized raster data.

## Stage E — Risk engine
Deterministic signals first. AI may summarize/explain later, but should not be the sole control that
marks an email safe or dangerous.
