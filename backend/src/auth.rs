use chrono::TimeZone;
use std::sync::Arc;
use tokio::sync::RwLock;

use cache_control::CacheControl;
use color_eyre::{eyre::eyre, Result};
use jsonwebtoken::{jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tracing::debug;
use url::Url;

struct JwksCached {
    jwks_uri: Url,
    pub jwks: JwkSet,
    invalidate_time: Option<chrono::DateTime<chrono::Utc>>,
}

fn calculate_invalidation_time(
    headers: &reqwest::header::HeaderMap,
) -> Option<chrono::DateTime<chrono::Utc>> {
    // Check if there's an expires header
    if let Some(expiry) = headers.get(reqwest::header::EXPIRES).and_then(|h| {
        h.to_str().ok().and_then(|h| {
            chrono::NaiveDateTime::parse_from_str(h, "%a, %d %b %Y %H:%M:%S GMT")
                .ok()
                .map(|d| chrono::Utc.from_utc_datetime(&d))
        })
    }) {
        return Some(expiry);
    }

    // Check if there's a cache control header
    let cache_control = match headers
        .get(reqwest::header::CACHE_CONTROL)
        .map(|c| c.to_str().map(CacheControl::from_value))
    {
        Some(Ok(Some(cache_control))) => cache_control,
        _ => return Some(chrono::Utc::now() + chrono::Duration::hours(1)),
    };

    // Don't cache if no-cache
    if let Some(cache_control::Cachability::NoCache) = cache_control.cachability {
        return None;
    }

    let age = match headers.get(reqwest::header::AGE).map(|a| a.to_str()) {
        Some(Ok(age)) => age.parse::<u64>().unwrap_or(0),
        _ => 0,
    };

    if let Some(Ok(max_age)) = cache_control.max_age.map(chrono::Duration::from_std) {
        return Some(chrono::Utc::now() + max_age - chrono::Duration::seconds(age as i64));
    }

    Some(chrono::Utc::now() + chrono::Duration::hours(1))
}

impl JwksCached {
    async fn new(authority: Box<str>) -> Result<Self> {
        let jwks_uri = format!("{}.well-known/jwks.json", authority);
        let jwks_uri = Url::parse(&jwks_uri).map_err(|_| {
            eyre!(
                "Malformed JWKS URI: `{}`. Did you forget a trailing `\\` for the authority?",
                jwks_uri
            )
        })?;

        let mut s = Self {
            jwks_uri,
            jwks: JwkSet { keys: vec![] },
            invalidate_time: None,
        };

        s.refresh_jwks()
            .await
            .map_err(|_| eyre!("Failed to fetch JWKS"))?;

        Ok(s)
    }

    pub fn find(&self, kid: &str) -> Option<&jsonwebtoken::jwk::Jwk> {
        self.jwks.find(kid)
    }

    pub fn is_fresh(&self) -> bool {
        if let Some(invalidate_time) = self.invalidate_time {
            invalidate_time > chrono::Utc::now()
        } else {
            false
        }
    }

    async fn refresh_jwks(&mut self) -> Result<()> {
        // Check if we need to refresh JWKS tokens
        debug!("Refreshing JWKS cache");
        let res = reqwest::get(self.jwks_uri.clone()).await?;

        let invalidate_time = calculate_invalidation_time(res.headers());
        let jwks = res.json::<JwkSet>().await?;

        self.jwks = jwks;
        self.invalidate_time = invalidate_time;

        if let Some(invalidate_time) = invalidate_time {
            debug!(
                "JWKS fetching successful. JWKS cache will be cached untill {}",
                invalidate_time
            );
        } else {
            debug!("JWKS fetching successful. JWKS cache will not be cached.");
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Auth0Validator {
    authority: Box<str>,
    audience: Box<str>,
    jwks: Arc<RwLock<JwksCached>>,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
}

impl Auth0Validator {
    pub async fn new(authority: Box<str>, audience: Box<str>) -> Result<Self> {
        let jwks = Arc::new(RwLock::new(JwksCached::new(authority.clone()).await?));

        Ok(Self {
            authority,
            audience,
            jwks,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        // Validate token
        let token_header = jsonwebtoken::decode_header(token)?;
        let kid = token_header.kid.ok_or_else(|| eyre!("Missing kid"))?;

        // Check if we need to refresh JWKS tokens
        let jwk = {
            let jwks = self.jwks.read().await;
            if jwks.is_fresh() {
                jwks.find(&kid).ok_or_else(|| eyre!("Invalid kid"))?.clone()
            } else {
                drop(jwks);
                let mut jwks = self.jwks.write().await;
                jwks.refresh_jwks().await?;
                jwks.find(&kid).ok_or_else(|| eyre!("Invalid kid"))?.clone()
            }
        };
        let key = DecodingKey::from_jwk(&jwk)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.authority]);
        validation.set_audience(&[&self.audience]);

        let token = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;

        Ok(token.claims)
    }
}
