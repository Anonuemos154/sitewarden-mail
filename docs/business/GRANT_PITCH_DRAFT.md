# Grant pitch draft

## Working title
SiteWarden Mail — offene, lokale E-Mail-Sicherheits- und Verwaltungssoftware

## Problem
Modern mail products split the problem into separate tools: security gateways, browser extensions,
cleanup services and productivity clients. Users either grant cloud services extensive mailbox
access or remain dependent on opaque provider decisions. A legitimate sender account, thread
hijack or harmless-looking content can still reach the user even when classic spam filters work.

## Proposed solution
A local-first open-source mail client that connects directly to Gmail, Microsoft and standard
mailboxes. It analyzes provider authentication results, URLs and attachments in isolated workers,
reconstructs mail into an inert Safe View, and combines this with reversible inbox cleanup and
productivity functions. Raw mailbox content is not uploaded to project servers by default.

## Public value
- inspectable security;
- user control;
- privacy-by-design;
- interoperability across providers;
- free access for individuals;
- reusable open components for safe rendering, risk explanation and mail operations.

## Six-month prototype
1. Gmail + Microsoft read path;
2. encrypted local index;
3. Safe View;
4. deterministic security signals;
5. cleanup plans + undo;
6. attachment quarantine;
7. signed alpha releases;
8. documentation/threat model.

## What makes it different
The novelty is not another spam classifier. The design moves trust away from the original mail
rendering path and combines security, mailbox operations and productivity in one open local client.
