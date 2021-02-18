use std::fmt;

use log::{debug, error};
use wechaty_puppet::{ContactPayload, MessagePayload, MessageType, PuppetImpl};

use crate::{Contact, Entity, WechatyContext, WechatyError};

pub type Message<T> = Entity<T, MessagePayload>;

impl<T> Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    pub(crate) fn new(id: String, ctx: WechatyContext<T>, payload: Option<MessagePayload>) -> Self {
        debug!("create message {}", id);
        let payload = match payload {
            Some(_) => payload,
            None => match ctx.messages().get(&id) {
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
        debug!("message.is_ready(id = {})", self.id_);
        match self.payload_ {
            None => false,
            Some(_) => true,
        }
    }

    pub fn is_self(&self) -> bool {
        if !self.is_ready() {
            false
        } else {
            match self.ctx.id() {
                Some(id) => self.from().unwrap().id() == id,
                None => false,
            }
        }
    }

    pub(crate) async fn ready(&mut self) -> Result<(), WechatyError> {
        debug!("message.ready(id = {})", self.id_);
        if self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx.puppet();
            match puppet.message_payload(self.id()).await {
                Ok(payload) => {
                    self.ctx.messages().insert(self.id(), payload.clone());
                    self.payload_ = Some(payload.clone());
                    if !payload.from_id.is_empty() {
                        self.ctx.contact_load(payload.from_id.clone()).await;
                    }
                    if !payload.to_id.is_empty() {
                        self.ctx.contact_load(payload.to_id.clone()).await;
                    }
                    if !payload.room_id.is_empty() {
                        self.ctx.room_load(payload.room_id.clone()).await;
                    }
                    Ok(())
                }
                Err(e) => {
                    error!("Error occurred while syncing message {}: {}", self.id_, e);
                    Err(WechatyError::from(e))
                }
            }
        }
    }

    pub fn id(&self) -> String {
        debug!("message.id(id = {})", self.id_);
        self.id_.clone()
    }

    pub fn conversation_id(&self) -> Option<String> {
        debug!("message.conversation_id(id = {})", self.id_);
        if self.is_ready() {
            let payload = self.payload().unwrap();
            if !payload.room_id.is_empty() {
                Some(payload.room_id)
            } else if !payload.from_id.is_empty() {
                Some(payload.from_id)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn payload(&self) -> Option<MessagePayload> {
        debug!("message.payload(id = {})", self.id_);
        self.payload_.clone()
    }

    pub(crate) fn set_payload(&mut self, payload: Option<MessagePayload>) {
        debug!("message.set_payload(id = {}, payload = {:?})", self.id_, payload);
        self.payload_ = payload;
    }

    pub fn from(&self) -> Option<Contact<T>> {
        debug!("message.from(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(Contact::new(payload.from_id.clone(), self.ctx.clone(), None)),
            None => None,
        }
    }

    pub fn to(&self) -> Option<Contact<T>> {
        debug!("message.to(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(Contact::new(payload.to_id.clone(), self.ctx.clone(), None)),
            None => None,
        }
    }

    pub fn room(&self) -> Option<String> {
        unimplemented!()
    }

    pub fn message_type(&self) -> Option<MessageType> {
        debug!("message.message_type(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.message_type.clone()),
            None => None,
        }
    }

    pub fn text(&self) -> Option<String> {
        debug!("message.text(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(payload.text.clone()),
            None => None,
        }
    }
}

impl<T> fmt::Debug for Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Message({})", self)
    }
}

impl<T> fmt::Display for Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from = match self.from() {
            Some(contact) => format!("{}", contact),
            None => String::new(),
        };
        let to = match self.to() {
            Some(contact) => format!("{}", contact),
            None => String::new(),
        };
        let message_type = match self.message_type() {
            Some(message_type) => format!("{:?}", message_type),
            None => String::new(),
        };
        let text = if self.is_ready() && self.message_type().unwrap() == MessageType::Text {
            let text = self.text().unwrap().chars().collect::<Vec<_>>();
            let len = text.len().min(70);
            format!(", Text: {}", text[0..len].iter().collect::<String>())
        } else {
            String::new()
        };
        write!(fmt, "From: {}, To: {}, Type: {}{}", from, to, message_type, text)
    }
}
