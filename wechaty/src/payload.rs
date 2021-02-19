use wechaty_puppet::{EventDongPayload, PuppetImpl, ScanStatus};

use crate::user::contact_self::ContactSelf;
use crate::{Contact, Message};

pub type DongPayload = EventDongPayload;

#[derive(Clone, Debug)]
pub struct LoginPayload<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub contact: ContactSelf<T>,
}

#[derive(Clone, Debug)]
pub struct LogoutPayload<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub contact: ContactSelf<T>,
}

#[derive(Clone, Debug)]
pub struct MessagePayload<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub message: Message<T>,
}

#[derive(Clone, Debug)]
pub struct ScanPayload {
    pub qrcode: Option<String>,
    pub status: ScanStatus,
}

impl ScanPayload {
    pub fn new(qrcode: Option<String>, status: ScanStatus) -> Self {
        Self { qrcode, status }
    }
}
