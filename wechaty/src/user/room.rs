use std::fmt;

use log::{debug, error, trace};
use wechaty_puppet::{PuppetImpl, RoomPayload};

use crate::{Entity, WechatyContext, WechatyError};

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
            ctx,
            payload_: payload,
        }
    }

    fn is_ready(&self) -> bool {
        trace!("message.is_ready(id = {})", self.id_);
        match self.payload_ {
            None => false,
            Some(_) => true,
        }
    }

    pub(crate) async fn ready(&mut self) -> Result<(), WechatyError> {
        debug!("room.ready(id = {})", self.id_);
        if self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx.puppet();
            match puppet.room_payload(self.id()).await {
                Ok(payload) => {
                    self.ctx.rooms().insert(self.id(), payload.clone());
                    self.payload_ = Some(payload.clone());
                    Ok(())
                }
                Err(e) => {
                    error!("Error occurred while syncing room {}: {}", self.id_, e);
                    Err(WechatyError::from(e))
                }
            }
        }
    }

    pub fn id(&self) -> String {
        trace!("room.id(id = {})", self.id_);
        self.id_.clone()
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
