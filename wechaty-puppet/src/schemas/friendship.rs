use num::FromPrimitive;
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Clone, PartialEq, FromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum FriendshipType {
    Unknown,
    Confirm,
    Receive,
    Verify,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum FriendshipSceneType {
    Unknown = 0,
    QQ = 1,
    Email = 2,
    Weixin = 3,
    QQtbd = 12,
    Room = 14,
    Phone = 15,
    Card = 17,
    Location = 18,
    Bottle = 25,
    Shaking = 29,
    QRCode = 30,
}

// TODO: In Rust, a struct is not allowed to have a fixed value as its field.
#[derive(Debug, Clone)]
pub enum FriendshipPayload {
    Confirm {
        id: String,
        contact_id: String,
        hello: Option<String>,
        timestamp: u64,
        friendship_type: FriendshipType,
    },
    Receive {
        id: String,
        contact_id: String,
        hello: Option<String>,
        timestamp: u64,
        scene: Option<FriendshipSceneType>,
        stranger: Option<String>,
        ticket: String,
        friendship_type: FriendshipType,
    },
    Verify {
        id: String,
        contact_id: String,
        hello: Option<String>,
        timestamp: u64,
        friendship_type: FriendshipType,
    },
}

#[derive(Debug, Clone)]
pub struct FriendshipSearchQueryFilter {
    phone: Option<String>,
    weixin: Option<String>,
}
