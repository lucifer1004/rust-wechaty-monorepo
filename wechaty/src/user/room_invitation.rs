use wechaty_puppet::RoomInvitationPayload;

use crate::Entity;

pub type RoomInvitation<T> = Entity<T, RoomInvitationPayload>;
