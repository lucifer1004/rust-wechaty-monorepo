use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlLinkPayload {
    description: Option<String>,
    thumbnail_url: Option<String>,
    title: String,
    url: String,
}
