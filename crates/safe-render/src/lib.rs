use domain::{MessageEnvelope, SafeDocument, SafeLink};

pub fn render(message: &MessageEnvelope) -> SafeDocument {
    let mut links = Vec::new();

    for observed in &message.links {
        match url::Url::parse(&observed.raw_target) {
            Ok(url) if matches!(url.scheme(), "http" | "https" | "mailto") => {
                let host_ascii = url.host_str().unwrap_or("").to_string();
                links.push(SafeLink {
                    label: observed.visible_text.clone(),
                    scheme: url.scheme().to_string(),
                    host_display: host_ascii.clone(),
                    host_ascii,
                    path: url.path().to_string(),
                    // Safe View starts non-clickable. A separate explicit action may open a URL.
                    clickable: false,
                });
            }
            _ => {}
        }
    }

    let mut blocks = Vec::new();
    if let Some(text) = &message.plain_text {
        blocks.push(text.clone());
    } else if message.raw_html.is_some() {
        blocks.push("[HTML-Inhalt vorhanden. Für die Safe View muss zuerst eine isolierte Text-/Pixel-Rekonstruktion erfolgen.]".into());
    }

    SafeDocument {
        subject: message.subject.clone(),
        sender_display: message.from.address.clone(),
        plain_blocks: blocks,
        links,
        images: vec![],
        warnings: vec![],
    }
}
