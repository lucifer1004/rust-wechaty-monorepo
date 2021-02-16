#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum ImageType {
    Unknown,
    Thumbnail,
    HD,
    Artwork,
}
