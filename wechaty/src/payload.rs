use wechaty_puppet::{EventDongPayload, PuppetImpl};

use crate::{ContactSelf, Message};

pub type DongPayload = EventDongPayload;

pub type LoginPayload<T> = ContactSelf<T>;

#[derive(Clone, Debug)]
pub struct MessagePayload {
    pub message: Message,
}
