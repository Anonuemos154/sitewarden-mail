use domain::{MessageEnvelope, RiskReport, SafeDocument};

pub struct ScanOutcome {
    pub risk: RiskReport,
    pub safe_document: SafeDocument,
}

pub fn scan(message: &MessageEnvelope) -> ScanOutcome {
    // Future stages:
    // 1. header/authentication
    // 2. identity / display-name / homograph analysis
    // 3. URL canonicalization and local reputation
    // 4. attachment metadata and sandbox worker results
    // 5. thread anomaly detection
    // 6. deterministic policy engine
    // 7. safe reconstruction
    let risk = risk_engine::analyze(message);
    let safe_document = safe_render::render(message);
    ScanOutcome {
        risk,
        safe_document,
    }
}
