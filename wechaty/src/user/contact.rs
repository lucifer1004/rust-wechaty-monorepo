use std::fmt;

use log::{debug, error};
use wechaty_puppet::{ContactPayload, ContactQueryFilter, PayloadType, Puppet, PuppetError, PuppetImpl};

use crate::WechatyContext;

#[derive(Clone)]
pub struct Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    ctx: WechatyContext<T>,
    id: String,
    payload: Option<ContactPayload>,
}

impl<T> Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>) -> Self {
        debug!("create contact {}", id);
        Self { id, ctx, payload: None }
    }

    fn is_ready(&self) -> bool {
        debug!("contact.is_ready(id = {})", self.id);
        match self.payload {
            None => false,
            Some(_) => true,
        }
    }

    pub(crate) async fn ready(&mut self, force_sync: bool) -> Result<(), PuppetError> {
        debug!("contact.ready(id = {}, force_sync = {})", self.id, force_sync);
        if !force_sync && self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx.get_puppet();
            if force_sync {
                if let Err(e) = puppet.dirty_payload(PayloadType::Contact, self.id.clone()).await {
                    error!("Error occurred while syncing contact {}: {}", self.id, e);
                    return Err(e);
                }
            }
            match puppet.contact_payload(self.id.clone()).await {
                Ok(payload) => {
                    self.payload = Some(payload);
                    Ok(())
                }
                Err(e) => {
                    error!("Error occurred while syncing contact {}: {}", self.id, e);
                    Err(e)
                }
            }
        }
    }

    pub(crate) async fn sync(&mut self) -> Result<(), PuppetError> {
        debug!("contact.sync(id = {})", self.id);
        self.ready(true).await
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_payload(&self) -> Option<ContactPayload> {
        self.payload.clone()
    }

    pub(crate) fn set_payload(&mut self, payload: Option<ContactPayload>) {
        self.payload = payload;
    }

    pub fn get_name(&self) -> String {
        match &self.payload {
            Some(payload) => payload.name.clone(),
            None => String::new(),
        }
    }

    pub async fn say(&mut self) {}
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
        let identity = if let Some(payload) = &self.payload {
            if !payload.alias.is_empty() {
                payload.alias.clone()
            } else if !payload.name.is_empty() {
                payload.name.clone()
            } else if !self.id.is_empty() {
                self.id.clone()
            } else {
                "loading...".to_owned()
            }
        } else {
            "loading...".to_owned()
        };
        write!(fmt, "{}", identity)
    }
}
