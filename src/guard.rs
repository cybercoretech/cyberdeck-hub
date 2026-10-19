//! Request guard for the loopback-only hub.
//!
//! The hub runs diagnostics and archives files on request, so a web page in
//! the user's browser must not be able to drive it:
//!
//! - every request must carry a loopback `Host` for our port (blocks DNS
//!   rebinding: a rebound request carries the attacker's hostname);
//! - state-changing requests must not come from another site:
//!   `Sec-Fetch-Site` must be `same-origin` or `none`, and any `Origin` must
//!   be our own. Requests with neither header (curl, the Mission Control
//!   panel) are allowed — they aren't a browser acting on a page's behalf;
//! - a request with this run's local-token/v1 bearer (see `localtoken`;
//!   what Nightwatch's gateway sends) skips the browser checks.
//!
//! Same rules as Dockspace's guard.

use crate::localtoken::LocalToken;
use axum::{
    extract::Request,
    http::{header, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::OnceLock;

static PORT: OnceLock<u16> = OnceLock::new();
static LOCAL: OnceLock<LocalToken> = OnceLock::new();

/// Call once at startup, before serving.
pub fn init(port: u16, local: LocalToken) {
    let _ = PORT.set(port);
    let _ = LOCAL.set(local);
}

fn allowed_hosts(port: u16) -> [String; 3] {
    [
        format!("127.0.0.1:{port}"),
        format!("localhost:{port}"),
        format!("[::1]:{port}"),
    ]
}

/// Pure decision function (unit-tested below).
pub fn check(
    method: &Method,
    host: Option<&str>,
    origin: Option<&str>,
    sec_fetch_site: Option<&str>,
    local_token: bool,
    port: u16,
) -> Result<(), &'static str> {
    let hosts = allowed_hosts(port);
    let Some(host) = host else {
        return Err("missing Host header");
    };
    if !hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
        return Err("Host is not this loopback hub");
    }
    if local_token || matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS) {
        return Ok(());
    }
    if let Some(site) = sec_fetch_site {
        if !matches!(site, "same-origin" | "none") {
            return Err("cross-site request refused");
        }
    }
    if let Some(origin) = origin {
        if !hosts
            .iter()
            .any(|h| origin.eq_ignore_ascii_case(&format!("http://{h}")))
        {
            return Err("cross-origin request refused");
        }
    }
    Ok(())
}

pub async fn middleware(req: Request, next: Next) -> Response {
    let port = *PORT.get().unwrap_or(&8080);
    let h = req.headers();
    let get = |name: header::HeaderName| h.get(name).and_then(|v| v.to_str().ok());
    let local = LOCAL.get().is_some_and(|l| l.verify(h));
    let verdict = check(
        req.method(),
        get(header::HOST),
        get(header::ORIGIN),
        h.get("sec-fetch-site").and_then(|v| v.to_str().ok()),
        local,
        port,
    );
    match verdict {
        Ok(()) => {
            if local {
                if let Some(actor) = LocalToken::actor(h) {
                    tracing::info!("{} {} via {actor}", req.method(), req.uri().path());
                }
            }
            next.run(req).await
        }
        Err(reason) => {
            tracing::warn!("guard refused {} {}: {reason}", req.method(), req.uri().path());
            (StatusCode::FORBIDDEN, reason).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u16 = 8080;
    const H: Option<&str> = Some("127.0.0.1:8080");

    #[test]
    fn own_page_and_header_less_clients_are_allowed() {
        assert!(check(&Method::POST, H, Some("http://127.0.0.1:8080"), Some("same-origin"), false, P).is_ok());
        assert!(check(&Method::POST, H, None, None, false, P).is_ok()); // curl
        assert!(check(&Method::GET, H, None, Some("cross-site"), false, P).is_ok());
    }

    #[test]
    fn cross_site_writes_are_refused_without_the_local_token() {
        assert!(check(&Method::POST, H, Some("https://evil.example"), Some("cross-site"), false, P).is_err());
        assert!(check(&Method::POST, H, Some("https://evil.example"), None, false, P).is_err());
        assert!(check(&Method::POST, H, Some("https://evil.example"), Some("cross-site"), true, P).is_ok());
    }

    #[test]
    fn dns_rebinding_host_is_refused_even_with_the_token() {
        assert!(check(&Method::GET, Some("evil.example:8080"), None, None, false, P).is_err());
        assert!(check(&Method::POST, Some("evil.example:8080"), None, None, true, P).is_err());
        assert!(check(&Method::GET, None, None, None, false, P).is_err());
        assert!(check(&Method::GET, Some("127.0.0.1:9999"), None, None, false, P).is_err());
    }
}
