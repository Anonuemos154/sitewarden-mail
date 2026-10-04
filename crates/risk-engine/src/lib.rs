use domain::{AuthResult, MessageEnvelope, RiskReport, RiskSignal, Severity};
use time::OffsetDateTime;
use uuid::Uuid;

fn points(sev: &Severity) -> u8 {
    match sev {
        Severity::Info => 0,
        Severity::Low => 8,
        Severity::Medium => 20,
        Severity::High => 40,
        Severity::Critical => 70,
    }
}

pub fn analyze(message: &MessageEnvelope) -> RiskReport {
    let mut signals = Vec::new();

    if matches!(message.auth.dmarc, AuthResult::Fail) {
        signals.push(RiskSignal {
            code: "auth.dmarc.fail".into(),
            severity: Severity::High,
            title: "DMARC-Prüfung fehlgeschlagen".into(),
            explanation: "Die Authentifizierungsinformation des Providers meldet DMARC=fail."
                .into(),
            evidence: vec![],
        });
    }

    if let Some(reply_to) = &message.reply_to {
        let from_domain = message.from.address.rsplit('@').next().unwrap_or_default();
        let reply_domain = reply_to.address.rsplit('@').next().unwrap_or_default();
        if !from_domain.eq_ignore_ascii_case(reply_domain) {
            signals.push(RiskSignal {
                code: "identity.reply_to_mismatch".into(),
                severity: Severity::Medium,
                title: "Reply-To weicht vom Absender ab".into(),
                explanation: "Antworten würden an eine andere Domain gesendet.".into(),
                evidence: vec![from_domain.to_string(), reply_domain.to_string()],
            });
        }
    }

    for link in &message.links {
        let visible = link.visible_text.trim().to_lowercase();
        let target = link.raw_target.trim().to_lowercase();
        if visible.starts_with("http") && !target.starts_with(&visible) {
            signals.push(RiskSignal {
                code: "url.visible_target_mismatch".into(),
                severity: Severity::Medium,
                title: "Sichtbarer Link und Ziel unterscheiden sich".into(),
                explanation:
                    "Der Linktext sieht wie eine URL aus, zeigt aber auf ein anderes Ziel.".into(),
                evidence: vec![link.visible_text.clone(), link.raw_target.clone()],
            });
        }
    }

    let score = signals
        .iter()
        .map(|s| points(&s.severity) as u16)
        .sum::<u16>()
        .min(100) as u8;

    RiskReport {
        report_id: Uuid::new_v4(),
        message_id: message.message_id.clone(),
        score,
        signals,
        generated_at: OffsetDateTime::now_utc(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_capped_at_100() {
        let severities = [Severity::Critical, Severity::Critical];
        let total = severities
            .iter()
            .map(points)
            .map(u16::from)
            .sum::<u16>()
            .min(100);
        assert_eq!(total, 100);
    }
}
