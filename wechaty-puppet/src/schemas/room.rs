use regex::Regex;
use wechaty_grpc::puppet::{RoomPayloadResponse, RoomMemberPayloadResponse};

#[derive(Debug, Clone)]
pub struct RoomMemberQueryFilter {
    name: Option<String>,
    room_alias: Option<String>,
    contact_alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RoomQueryFilter {
    pub id: Option<String>,
    pub topic: Option<String>,
    pub topic_regex: Option<Regex>,
}

#[derive(Debug, Clone)]
pub struct RoomPayload {
    pub id: String,
    pub topic: String,
    pub avatar: String,
    pub member_id_list: Vec<String>,
    pub owner_id: String,
    pub admin_id_list: Vec<String>,
}

impl From<RoomPayloadResponse> for RoomPayload {
    fn from(response: RoomPayloadResponse) -> Self {
        Self {
            id: response.id,
            topic: response.topic,
            avatar: response.avatar,
            member_id_list: response.member_ids,
            owner_id: response.owner_id,
            admin_id_list: response.admin_ids,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomMemberPayload {
    pub id: String,
    pub room_alias: String,
    pub inviter_id: String,
    pub avatar: String,
    pub name: String,
}

impl From<RoomMemberPayloadResponse> for RoomMemberPayload {
    fn from(response: RoomMemberPayloadResponse) -> Self {
        Self {
            id: response.id,
            room_alias: response.room_alias,
            avatar: response.avatar,
            inviter_id: response.inviter_id,
            name: response.name,
        }
    }
}

// FIXME: trait aliases are experimental, see issue #41517 <https://github.com/rust-lang/rust/issues/41517>
// pub trait RoomPayloadFilterFunction = Fn(RoomPayload) -> bool;
//
// pub trait RoomPayloadFilterFactory = Fn(RoomQueryFilter) -> RoomPayloadFilterFunction;
