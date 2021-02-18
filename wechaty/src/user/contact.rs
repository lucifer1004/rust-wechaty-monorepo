use std::fmt;

use log::{debug, error, info};
use wechaty_puppet::{
    ContactGender, ContactPayload, ContactQueryFilter, FileBox, MiniProgramPayload, PayloadType, PuppetImpl,
    UrlLinkPayload,
};

use crate::user::entity::Entity;
use crate::{Message, WechatyContext, WechatyError};

pub type Contact<T> = Entity<T, ContactPayload>;

impl<T> Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
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

    fn is_ready(&self) -> bool {
        debug!("contact.is_ready(id = {})", self.id_);
        match self.payload_ {
            None => false,
            Some(_) => true,
        }
    }

    pub(crate) async fn ready(&mut self, force_sync: bool) -> Result<(), WechatyError> {
        debug!("contact.ready(id = {}, force_sync = {})", self.id_, force_sync);
        if !force_sync && self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx.puppet();
            if force_sync {
                if let Err(e) = puppet.dirty_payload(PayloadType::Contact, self.id_.clone()).await {
                    error!("Error occurred while syncing contact {}: {}", self.id_, e);
                    return Err(WechatyError::from(e));
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
                    Err(WechatyError::from(e))
                }
            }
        }
    }

    pub(crate) async fn sync(&mut self) -> Result<(), WechatyError> {
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

    pub async fn set_alias(&self, new_alias: String) -> Result<(), WechatyError> {
        debug!("contact.set_alias(id = {}, new_alias = {})", self.id_, new_alias);
        match self.ctx.puppet().contact_alias_set(self.id(), new_alias.clone()).await {
            Err(e) => {
                error!("Failed to set alias for {}, reason: {}", self, e);
                Err(WechatyError::from(e))
            }
            Ok(_) => {
                if let Err(e) = self.ctx.puppet().dirty_payload(PayloadType::Contact, self.id()).await {
                    error!("Failed to dirty payload for {}, reason: {}", self, e);
                }
                match self.ctx.puppet().contact_payload(self.id()).await {
                    Ok(payload) => {
                        if payload.alias != new_alias {
                            error!("Payload is not correctly set.");
                        }
                    }
                    Err(e) => {
                        error!("Failed to verify payload for {}, reason: {}", self, e);
                    }
                };
                Ok(())
            }
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

    async fn message_load(&mut self, message_id: String) -> Result<Option<Message<T>>, WechatyError> {
        match self.ctx.message_load(message_id).await {
            Ok(message) => {
                info!("Message sent: {}", message);
                Ok(Some(message))
            }
            Err(e) => {
                error!(
                    "Message has been sent to {} but cannot get message payload, reason: {}",
                    self, e
                );
                Ok(None)
            }
        }
    }

    pub async fn send_text(&mut self, text: String) -> Result<Option<Message<T>>, WechatyError> {
        let message_id = match self.ctx.puppet().message_send_text(self.id(), text, vec![]).await {
            Ok(Some(id)) => id,
            Ok(None) => {
                error!("Message has been sent to {} but cannot get message id", self);
                return Ok(None);
            }
            Err(e) => return Err(WechatyError::from(e)),
        };
        self.message_load(message_id).await
    }

    pub async fn send_contact(&mut self, contact_id: String) -> Result<Option<Message<T>>, WechatyError> {
        let message_id = match self.ctx.puppet().message_send_contact(self.id(), contact_id).await {
            Ok(Some(id)) => id,
            Ok(None) => {
                error!("Message has been sent to {} but cannot get message id", self);
                return Ok(None);
            }
            Err(e) => return Err(WechatyError::from(e)),
        };
        self.message_load(message_id).await
    }

    pub async fn send_file(&mut self, file: FileBox) -> Result<Option<Message<T>>, WechatyError> {
        let message_id = match self.ctx.puppet().message_send_file(self.id(), file).await {
            Ok(Some(id)) => id,
            Ok(None) => {
                error!("Message has been sent to {} but cannot get message id", self);
                return Ok(None);
            }
            Err(e) => return Err(WechatyError::from(e)),
        };
        self.message_load(message_id).await
    }

    pub async fn send_mini_program(
        &mut self,
        mini_program: MiniProgramPayload,
    ) -> Result<Option<Message<T>>, WechatyError> {
        let message_id = match self
            .ctx
            .puppet()
            .message_send_mini_program(self.id(), mini_program)
            .await
        {
            Ok(Some(id)) => id,
            Ok(None) => {
                error!("Message has been sent to {} but cannot get message id", self);
                return Ok(None);
            }
            Err(e) => return Err(WechatyError::from(e)),
        };
        self.message_load(message_id).await
    }

    pub async fn send_url(&mut self, url: UrlLinkPayload) -> Result<Option<Message<T>>, WechatyError> {
        let message_id = match self.ctx.puppet().message_send_url(self.id(), url).await {
            Ok(Some(id)) => id,
            Ok(None) => {
                error!("Message has been sent to {} but cannot get message id", self);
                return Ok(None);
            }
            Err(e) => return Err(WechatyError::from(e)),
        };
        self.message_load(message_id).await
    }
}

impl<T> fmt::Debug for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Contact({})", self)
    }
}

impl<T> fmt::Display for Contact<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let identity = match &self.payload_ {
            Some(payload) => {
                if !payload.alias.is_empty() {
                    payload.alias.clone()
                } else if !payload.name.is_empty() {
                    payload.name.clone()
                } else if !self.id_.is_empty() {
                    self.id_.clone()
                } else {
                    "loading...".to_owned()
                }
            }
            None => "loading...".to_owned(),
        };
        write!(fmt, "{}", identity)
    }
}
