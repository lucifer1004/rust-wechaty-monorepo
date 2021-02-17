use wechaty_puppet::EventDongPayload;

use crate::{ContactSelf, Message};

pub type DongPayload = EventDongPayload;

#[derive(Clone, Debug)]
pub struct LoginPayload {
    pub contact_self: ContactSelf,
}
#[derive(Clone, Debug)]
pub struct MessagePayload {
    pub message: Message,
}
