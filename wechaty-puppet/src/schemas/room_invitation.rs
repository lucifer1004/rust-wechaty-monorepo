use wechaty_grpc::puppet::RoomInvitationPayloadResponse;

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

impl From<RoomInvitationPayloadResponse> for RoomInvitationPayload {
    fn from(response: RoomInvitationPayloadResponse) -> Self {
        Self {
            id: response.id,
            inviter_id: response.inviter_id,
            topic: response.topic,
            avatar: response.avatar,
            invitation: response.invitation,
            member_count: response.member_count,
            member_id_list: response.member_ids,
            timestamp: response.timestamp,
            receiver_id: response.receiver_id,
        }
    }
}
