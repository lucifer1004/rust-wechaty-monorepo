use regex::Regex;

#[derive(Debug, Clone)]
pub struct RoomMemberQueryFilter {
    name: Option<String>,
    room_alias: Option<String>,
    contact_alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RoomQueryFilter {
    id: Option<String>,
    topic: Option<String>,
    topic_regex: Option<Regex>,
}

#[derive(Debug, Clone)]
pub struct RoomPayload {
    id: String,
    topic: String,
    avatar: Option<String>,
    member_id_list: Vec<String>,
    owner_id: String,
    admin_id_list: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RoomMemberPayload {
    id: String,
    room_alias: Option<String>,
    inviter_id: Option<String>,
    avatar: String,
    name: String,
}

pub type RoomPayloadFilterFunction = fn(RoomPayload) -> bool;

pub type RoomPayloadFilterFactory = fn(RoomQueryFilter) -> RoomPayloadFilterFunction;