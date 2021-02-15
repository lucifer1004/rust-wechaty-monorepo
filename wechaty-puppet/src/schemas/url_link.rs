#[derive(Debug, Clone)]
pub struct UrlLinkPayload {
    description: Option<String>,
    thumbnail_url: Option<String>,
    title: String,
    url: String,
}