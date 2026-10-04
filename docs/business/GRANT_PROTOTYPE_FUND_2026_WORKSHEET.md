# Prototype Fund 2026 — application worksheet

> Current published deadline: **30 November 2026**.
> The exact applicant eligibility must be confirmed before submission.

## Best thematic line

Likely strongest fit: **Innovation** or **Resilienz**, depending on the final project framing and
applicant eligibility.

## Working title

**SiteWarden Mail — Freie lokale E-Mail-Sicherheits- und Verwaltungssoftware**

## Kurzbeschreibung

SiteWarden Mail ist ein freier, lokal arbeitender E-Mail-Client mit Sicherheits- und
Verwaltungsfunktionen. Die Software soll Nachrichten aus Gmail, Microsoft 365 und offenen
Mailstandards zusammenführen, technische Risiken nachvollziehbar analysieren und Inhalte in einer
reduzierten Safe View darstellen, ohne ursprüngliches aktives Mail-HTML direkt in die vertrauenswürdige
Oberfläche zu übernehmen.

## Problem

Viele Mailwerkzeuge lösen jeweils nur einen Teil:
- Spam/Phishing-Filter beim Provider;
- proprietäre Produktivitäts-Erweiterungen;
- Cloud-Cleanup-Dienste;
- separate Endpoint-Security.

Dadurch bleiben Nutzer:innen abhängig von proprietären Entscheidungen und geben teilweise sehr
umfangreichen Cloud-Diensten Zugriff auf Postfächer.

## Idee

Ein offener, lokaler Client verbindet:
- Multi-Provider-Mail;
- erklärbare Sicherheitsindikatoren;
- Safe Rendering;
- isolierte Attachment-Prüfung;
- reversible Inbox-Bereinigung;
- Produktivitätsfunktionen.

Die Software soll ohne verpflichtenden Projekt-Cloud-Dienst funktionieren.

## Gesellschaftlicher Nutzen

- mehr Kontrolle über persönliche Kommunikation;
- nachvollziehbare statt rein proprietäre Sicherheitsentscheidungen;
- geringere Abhängigkeit von einem einzelnen Mailanbieter;
- Datenschutz durch lokale Verarbeitung;
- freie Software, die geprüft und weiterentwickelt werden kann;
- potenzieller Nutzen für Privatpersonen, kleine Unternehmen und zivilgesellschaftliche
  Organisationen.

## Prototyp-Ziel

Innerhalb des Förderzeitraums soll ein technisch glaubwürdiger Open-Source-Prototyp entstehen mit:

1. mindestens zwei echten Provider-Anbindungen;
2. verschlüsseltem lokalen Index;
3. Safe View;
4. URL-/Identitätsanalyse;
5. isoliertem Attachment-Worker;
6. Cleanup-Plan/Undo;
7. automatisierten Tests/Fuzzing;
8. dokumentiertem Threat Model;
9. signierbarem Alpha-Release-Prozess.

## Abgrenzung

Nicht Ziel:
- Versprechen vollständiger Angriffserkennung;
- proprietärer zentraler Maildienst;
- automatische endgültige Löschung;
- KI als alleinige Sicherheitsentscheidung.

## Open Source

Repository:
https://github.com/Anonuemos154/sitewarden-mail

Lizenz:
AGPL-3.0-or-later

## Was bereits existiert

- öffentliche Alpha-Seiten;
- Open-Source-Repository;
- CI/CodeQL;
- Desktop-/Web-Grundlage;
- Security-/Safe-Render-/Cleanup-Architektur;
- Threat Model und Security Invariants.

## Was mit Förderung gebaut werden soll

Der Antrag sollte sich auf einen klaren technischen Sprung konzentrieren, z. B.:

**"Von der Architektur zum isolierten, interoperablen Mail-Sicherheitsprototyp."**

Dafür:
- echte Provider-Anbindung;
- MIME-/Attachment-Sandbox;
- Safe-Reconstruction;
- lokaler Vault;
- Testcorpus;
- Release-Hardening.

## Applicant gate

Vor Antrag:
- aktuelle Förderbedingungen lesen;
- prüfen, ob die antragstellende Person / das Team zur Förderlinie passt;
- insbesondere nicht einfach eine Unternehmensbewerbung abschicken, wenn nur natürliche Personen /
  kleine Teams zugelassen sind.

## Noch einzutragen

- [ ] Name(n) der Antragstellenden;
- [ ] Alter/Ausbildungs-/Studienstatus, falls für gewählte Linie relevant;
- [ ] Wohn-/Arbeitsort;
- [ ] vorhandene Kompetenzen;
- [ ] konkrete sechsmonatige Zeitplanung;
- [ ] gewünschte Fördersumme;
- [ ] persönliche Motivation;
- [ ] ggf. Abgrenzung zu kommerziellen MSD/SiteWarden-Aktivitäten;
- [ ] Formulardaten und Erklärungen.

## Pitch-Satz

> E-Mail soll nicht sicher sein müssen, weil ein Filter jede Täuschung erkennt. Der Client soll
> vielmehr so gebaut sein, dass untrusted Mail-Inhalte möglichst wenig technische Autorität über
> die vertrauenswürdige Oberfläche erhalten.
