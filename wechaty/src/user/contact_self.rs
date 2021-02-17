use std::fmt;

use log::debug;
use wechaty_puppet::{ContactPayload, ContactQueryFilter, PayloadType, Puppet, PuppetError, PuppetImpl};

use crate::{Contact, WechatyContext};

#[derive(Clone)]
pub struct ContactSelf<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    contact: Contact<T>,
}

impl<T> ContactSelf<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>) -> Self {
        debug!("create contact self {}", id);
        let contact = Contact::new(id, ctx);
        Self { contact }
    }

    fn is_ready(&self) -> bool {
        debug!("contact_self.is_ready(id = {})", self.contact.get_id());
        match self.contact.get_payload() {
            None => false,
            Some(_) => true,
        }
    }

    pub async fn ready(&mut self, force_sync: bool) -> Result<(), PuppetError> {
        debug!(
            "contact_self.ready(id = {}, force_sync = {})",
            self.contact.get_id(),
            force_sync
        );
        self.contact.ready(force_sync).await
    }

    pub async fn sync(&mut self) -> Result<(), PuppetError> {
        debug!("contact_self.sync(id = {})", self.contact.get_id());
        self.contact.sync().await
    }
}

impl<T> fmt::Debug for ContactSelf<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "ContactSelf({})", self)
    }
}

impl<T> fmt::Display for ContactSelf<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.contact)
    }
}
