use std::time::{SystemTime, UNIX_EPOCH};

use num::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use wechaty_grpc::puppet::FriendshipPayloadResponse;

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
    Unknown,
    Confirm {
        id: String,
        contact_id: String,
        hello: String,
        timestamp: u64,
        friendship_type: FriendshipType,
    },
    Receive {
        id: String,
        contact_id: String,
        hello: String,
        timestamp: u64,
        scene: FriendshipSceneType,
        stranger: String,
        ticket: String,
        friendship_type: FriendshipType,
    },
    Verify {
        id: String,
        contact_id: String,
        hello: String,
        timestamp: u64,
        friendship_type: FriendshipType,
    },
}

impl From<FriendshipPayloadResponse> for FriendshipPayload {
    fn from(response: FriendshipPayloadResponse) -> Self {
        match response.r#type {
            1 => FriendshipPayload::Confirm {
                id: response.id,
                contact_id: response.contact_id,
                hello: response.hello,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                friendship_type: FriendshipType::Confirm,
            },
            2 => FriendshipPayload::Receive {
                id: response.id,
                contact_id: response.contact_id,
                hello: response.hello,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                scene: FromPrimitive::from_i32(response.scene).unwrap(),
                stranger: response.stranger,
                ticket: response.ticket,
                friendship_type: FriendshipType::Receive,
            },
            3 => FriendshipPayload::Verify {
                id: response.id,
                contact_id: response.contact_id,
                hello: response.hello,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                friendship_type: FriendshipType::Verify,
            },
            _ => FriendshipPayload::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FriendshipSearchQueryFilter {
    phone: Option<String>,
    weixin: Option<String>,
}
