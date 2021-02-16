use regex::Regex;

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

// FIXME: trait aliases are experimental, see issue #41517 <https://github.com/rust-lang/rust/issues/41517>
// pub trait RoomPayloadFilterFunction = Fn(RoomPayload) -> bool;
//
// pub trait RoomPayloadFilterFactory = Fn(RoomQueryFilter) -> RoomPayloadFilterFunction;
