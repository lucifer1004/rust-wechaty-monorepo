use std::fmt;

use log::{debug, error};
use wechaty_puppet::{ContactGender, ContactPayload, ContactQueryFilter, PayloadType, PuppetError, PuppetImpl};

use crate::WechatyContext;

#[derive(Clone)]
pub struct Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    ctx: WechatyContext<T>,
    id_: String,
    payload_: Option<ContactPayload>,
}

impl<T> Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>, payload: Option<ContactPayload>) -> Self {
        debug!("create contact {}", id);
        Self {
            id_: id,
            ctx,
            payload_: payload,
        }
    }

    fn is_ready(&self) -> bool {
        debug!("contact.is_ready(id = {})", self.id_);
        match self.payload_ {
            None => false,
            Some(_) => true,
        }
    }

    pub(crate) async fn ready(&mut self, force_sync: bool) -> Result<(), PuppetError> {
        debug!("contact.ready(id = {}, force_sync = {})", self.id_, force_sync);
        if !force_sync && self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx.puppet();
            if force_sync {
                if let Err(e) = puppet.dirty_payload(PayloadType::Contact, self.id_.clone()).await {
                    error!("Error occurred while syncing contact {}: {}", self.id_, e);
                    return Err(e);
                }
            }
            match puppet.contact_payload(self.id_.clone()).await {
                Ok(payload) => {
                    self.ctx.contacts().insert(self.id_.clone(), payload.clone());
                    self.payload_ = Some(payload);
                    Ok(())
                }
                Err(e) => {
                    error!("Error occurred while syncing contact {}: {}", self.id_, e);
                    Err(e)
                }
            }
        }
    }

    pub(crate) async fn sync(&mut self) -> Result<(), PuppetError> {
        debug!("contact.sync(id = {})", self.id_);
        self.ready(true).await
    }

    pub fn id(&self) -> String {
        debug!("contact.id(id = {})", self.id_);
        self.id_.clone()
    }

    pub fn payload(&self) -> Option<ContactPayload> {
        debug!("contact.payload(id = {})", self.id_);
        self.payload_.clone()
    }

    pub(crate) fn set_payload(&mut self, payload: Option<ContactPayload>) {
        debug!("contact.set_payload(id = {}, payload = {:?})", self.id_, payload);
        self.payload_ = payload;
    }

    pub fn name(&self) -> Option<String> {
        debug!("contact.name(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.name.clone()),
            None => None,
        }
    }

    pub fn gender(&self) -> Option<ContactGender> {
        debug!("contact.gender(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.gender.clone()),
            None => None,
        }
    }

    pub fn province(&self) -> Option<String> {
        debug!("contact.province(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.province.clone()),
            None => None,
        }
    }

    pub fn city(&self) -> Option<String> {
        debug!("contact.city(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.city.clone()),
            None => None,
        }
    }

    pub fn friend(&self) -> Option<bool> {
        debug!("contact.friend(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.friend),
            None => None,
        }
    }

    pub fn star(&self) -> Option<bool> {
        debug!("contact.star(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.star),
            None => None,
        }
    }

    pub fn alias(&self) -> Option<String> {
        debug!("contact.alias(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.alias.clone()),
            None => None,
        }
    }

    /// Check if current contact is the bot self.
    pub fn is_self(&self) -> bool {
        debug!("contact.is_self(id = {})", self.id_);
        match self.ctx.id() {
            Some(id) => self.id_ == id,
            None => false,
        }
    }

    pub async fn say(&mut self) {
        println!("HEY");
    }
}

impl<T> fmt::Debug for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Contact({})", self)
    }
}

impl<T> fmt::Display for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let identity = if let Some(payload) = &self.payload_ {
            if !payload.alias.is_empty() {
                payload.alias.clone()
            } else if !payload.name.is_empty() {
                payload.name.clone()
            } else if !self.id_.is_empty() {
                self.id_.clone()
            } else {
                "loading...".to_owned()
            }
        } else {
            "loading...".to_owned()
        };
        write!(fmt, "{}", identity)
    }
}
