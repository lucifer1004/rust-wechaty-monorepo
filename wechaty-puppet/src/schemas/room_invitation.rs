#[derive(Debug, Clone)]
pub struct RoomInvitationPayload {
    id: String,
    inviter_id: String,
    topic: String,
    avatar: String,
    invitation: String,
    member_count: u32,
    member_id_list: Vec<String>,
    timestamp: u64,
    receiver_id: String,
}
