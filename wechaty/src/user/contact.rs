use std::fmt;

use log::{debug, trace};
use wechaty_puppet::{ContactPayload, PuppetImpl};

use crate::user::entity::Entity;
use crate::{IntoContact, WechatyContext};

pub type Contact<T> = Entity<T, ContactPayload>;

impl<T> Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>, payload: Option<ContactPayload>) -> Self {
        debug!("create contact {}", id);
        let payload = match payload {
            Some(_) => payload,
            None => match ctx.contacts().get(&id) {
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
}

impl<T> IntoContact<T> for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn id(&self) -> String {
        trace!("contact.id(id = {})", self.id_);
        self.id_.clone()
    }

    fn ctx(&self) -> WechatyContext<T> {
        trace!("contact.ctx(id = {})", self.id_);
        self.ctx.clone()
    }

    fn identity(&self) -> String {
        trace!("contact.identity(id = {})", self.id_);
        match self.payload() {
            Some(payload) => {
                if !payload.alias.is_empty() {
                    payload.alias.clone()
                } else if !payload.name.is_empty() {
                    payload.name.clone()
                } else if !self.id().is_empty() {
                    self.id().clone()
                } else {
                    "loading...".to_owned()
                }
            }
            None => "loading...".to_owned(),
        }
    }

    fn payload(&self) -> Option<ContactPayload> {
        trace!("contact.payload(id = {})", self.id_);
        self.payload_.clone()
    }

    fn set_payload(&mut self, payload: Option<ContactPayload>) {
        debug!("contact.set_payload(id = {}, payload = {:?})", self.id_, payload);
        self.payload_ = payload;
    }
}

impl<T> fmt::Debug for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Contact({})", self)
    }
}

impl<T> fmt::Display for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.identity())
    }
}
