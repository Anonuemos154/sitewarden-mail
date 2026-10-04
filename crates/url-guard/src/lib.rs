use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UrlObservation {
    pub scheme: String,
    pub unicode_host: String,
    pub ascii_host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query_present: bool,
    pub has_userinfo: bool,
    pub ip_literal: bool,
}

pub fn inspect(raw: &str) -> Option<UrlObservation> {
    let parsed = url::Url::parse(raw).ok()?;
    if !matches!(parsed.scheme(), "http" | "https" | "mailto") {
        return None;
    }
    let host = parsed.host_str().unwrap_or_default();
    let (unicode_host, _) = idna::domain_to_unicode(host);
    let ascii_host = idna::domain_to_ascii(host).unwrap_or_else(|_| host.to_string());
    let has_userinfo = !parsed.username().is_empty() || parsed.password().is_some();
    let ip_literal = host.parse::<std::net::IpAddr>().is_ok();
    Some(UrlObservation {
        scheme: parsed.scheme().to_string(),
        unicode_host,
        ascii_host,
        port: parsed.port(),
        path: parsed.path().to_string(),
        query_present: parsed.query().is_some(),
        has_userinfo,
        ip_literal,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_javascript() {
        assert!(inspect("javascript:alert(1)").is_none());
    }
    #[test]
    fn detects_userinfo() {
        let x = inspect("https://paypal.example@evil.invalid/login").unwrap();
        assert!(x.has_userinfo);
        assert_eq!(x.ascii_host, "evil.invalid");
    }
}
