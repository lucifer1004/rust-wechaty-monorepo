#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum PayloadType {
    Unknown,
    Message,
    Contact,
    Room,
    RoomMember,
    Friendship,
}