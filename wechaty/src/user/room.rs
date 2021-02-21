use std::fmt;

use log::{debug, error};
use wechaty_puppet::{PayloadType, PuppetImpl, RoomPayload};

use crate::{Contact, Entity, WechatyContext, WechatyError};

pub type Room<T> = Entity<T, RoomPayload>;

impl<T> Room<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>, payload: Option<RoomPayload>) -> Self {
        debug!("create room {}", id);
        let payload = match payload {
            Some(_) => payload,
            None => match ctx.rooms().get(&id) {
                Some(payload) => Some(payload.clone()),
                None => None,
            },
        };
        Self {
            id_: id,
            ctx_: ctx,
            payload_: payload,
        }
    }

    pub(crate) async fn ready(&mut self, force_sync: bool) -> Result<(), WechatyError> {
        debug!("Room.ready(id = {})", self.id_);
        if !force_sync && self.is_ready() {
            Ok(())
        } else {
            let id = self.id();
            let mut puppet = self.ctx().puppet();
            if force_sync {
                if let Err(e) = puppet.dirty_payload(PayloadType::Room, id.clone()).await {
                    error!("Error occurred while dirtying room {}: {}", id, e);
                    return Err(WechatyError::from(e));
                }
                if let Err(e) = puppet.dirty_payload(PayloadType::RoomMember, id.clone()).await {
                    error!("Error occurred while dirtying members of room {}: {}", id, e);
                    return Err(WechatyError::from(e));
                }
            }
            match puppet.room_payload(id.clone()).await {
                Ok(payload) => {
                    self.ctx().rooms().insert(id, payload.clone());
                    self.set_payload(Some(payload.clone()));
                    self.ctx().contact_load_batch(payload.member_id_list).await;
                    Ok(())
                }
                Err(e) => {
                    error!("Error occurred while syncing contact {}: {}", id, e);
                    Err(WechatyError::from(e))
                }
            }
        }
    }

    pub(crate) async fn sync(&mut self) -> Result<(), WechatyError> {
        debug!("Room.sync(id = {})", self.id_);
        self.ready(true).await
    }

    pub async fn member_find_all(&self) -> Result<Vec<Contact<T>>, WechatyError> {
        debug!("Room.member_find_all(id = {})", self.id_);
        let ctx = self.ctx();
        match ctx.puppet().room_member_list(self.id()).await {
            Ok(member_id_list) => Ok(ctx.contact_load_batch(member_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }
}

impl<T> fmt::Debug for Room<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Room({})", self)
    }
}

impl<T> fmt::Display for Room<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let room_info = match &self.payload_ {
            Some(payload) => {
                if !payload.topic.is_empty() {
                    payload.topic.clone()
                } else if !self.id_.is_empty() {
                    self.id_.clone()
                } else {
                    "loading...".to_owned()
                }
            }
            None => "loading...".to_owned(),
        };
        write!(fmt, "{}", room_info)
    }
}
