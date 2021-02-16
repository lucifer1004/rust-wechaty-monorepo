use num::{FromPrimitive, ToPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ImageType {
    Unknown,
    Thumbnail,
    HD,
    Artwork,
}
