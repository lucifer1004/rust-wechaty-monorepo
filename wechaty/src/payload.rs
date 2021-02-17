use wechaty_puppet::{EventDongPayload, PuppetImpl};

use crate::{Contact, Message};

pub type DongPayload = EventDongPayload;

#[derive(Clone, Debug)]
pub struct LoginPayload<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub contact: Contact<T>,
}

#[derive(Clone, Debug)]
pub struct MessagePayload {
    pub message: Message,
}
