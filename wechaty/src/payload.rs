use wechaty_puppet::{EventDongPayload, PuppetImpl, ScanStatus};

use crate::user::contact_self::ContactSelf;
use crate::{Contact, Message, RoomInvitation, Room};

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

#[derive(Clone, Debug)]
pub struct ReadyPayload {}

#[derive(Clone, Debug)]
pub struct RoomInvitePayload<T>
    where
        T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub room_invitation: RoomInvitation<T>,
}

#[derive(Clone, Debug)]
pub struct RoomJoinPayload<T>
    where
        T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub room: Room<T>,
    pub invitee_list: Vec<Contact<T>>,
    pub inviter: Contact<T>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct RoomLeavePayload<T>
    where
        T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub room: Room<T>,
    pub removee_list: Vec<Contact<T>>,
    pub remover: Contact<T>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct RoomTopicPayload<T>
    where
        T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub room: Room<T>,
    pub old_topic: String,
    pub new_topic: String,
    pub changer: Contact<T>,
    pub timestamp: u64,
}