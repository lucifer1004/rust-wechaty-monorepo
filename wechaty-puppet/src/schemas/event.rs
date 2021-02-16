use crate::schemas::payload::PayloadType;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum ScanStatus {
    Unknown,
    Cancel,
    Waiting,
    Scanned,
    Confirmed,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct EventFriendshipPayload {
    friendship_id: String,
}

#[derive(Debug, Clone)]
pub struct EventLoginPayload {
    contact_id: String,
}

#[derive(Debug, Clone)]
pub struct EventLogoutPayload {
    contact_id: String,
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventMessagePayload {
    message_id: String,
}

#[derive(Debug, Clone)]
pub struct EventRoomInvitePayload {
    room_invitation_id: String,
}

#[derive(Debug, Clone)]
pub struct EventRoomJoinPayload {
    invitee_id_list: Vec<String>,
    inviter_id: String,
    room_id: String,
    timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct EventRoomLeavePayload {
    removee_id_list: Vec<String>,
    remover_id: String,
    room_id: String,
    timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct EventRoomTopicPayload {
    changer_id: String,
    new_topic: String,
    old_topic: String,
    room_id: String,
    timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct EventScanPayload {
    status: ScanStatus,
    qrcode: Option<String>,
    data: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventDongPayload {
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventErrorPayload {
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventReadyPayload {
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventResetPayload {
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventHeartbeatPayload {
    data: String,
}

#[derive(Debug, Clone)]
pub struct EventDirtyPayload {
    payload_type: PayloadType,
    payload_id: String,
}
