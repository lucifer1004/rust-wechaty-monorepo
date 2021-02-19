use std::fmt;

use log::{debug, error, trace};
use wechaty_puppet::{MessagePayload, MessageType, PuppetImpl};

use crate::{Contact, Entity, IntoContact, Room, WechatyContext, WechatyError};

pub type Message<T> = Entity<T, MessagePayload>;

impl<T> Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
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
            ctx_: ctx,
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

    pub fn is_self(&self) -> bool {
        trace!("message.is_self(id = {})", self.id_);
        if !self.is_ready() {
            false
        } else {
            self.from().unwrap().is_self()
        }
    }

    pub(crate) async fn ready(&mut self) -> Result<(), WechatyError> {
        debug!("message.ready(id = {})", self.id_);
        if self.is_ready() {
            Ok(())
        } else {
            let mut puppet = self.ctx_.puppet();
            match puppet.message_payload(self.id()).await {
                Ok(payload) => {
                    self.ctx_.messages().insert(self.id(), payload.clone());
                    self.payload_ = Some(payload.clone());
                    if !payload.from_id.is_empty() {
                        self.ctx_.contact_load(payload.from_id.clone()).await;
                    }
                    if !payload.to_id.is_empty() {
                        self.ctx_.contact_load(payload.to_id.clone()).await;
                    }
                    if !payload.room_id.is_empty() {
                        self.ctx_.room_load(payload.room_id.clone()).await;
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
        trace!("message.id(id = {})", self.id_);
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
        trace!("message.payload(id = {})", self.id_);
        self.payload_.clone()
    }

    pub(crate) fn set_payload(&mut self, payload: Option<MessagePayload>) {
        debug!("message.set_payload(id = {}, payload = {:?})", self.id_, payload);
        self.payload_ = payload;
    }

    pub fn from(&self) -> Option<Contact<T>> {
        debug!("message.from(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => {
                if !payload.from_id.is_empty() {
                    Some(Contact::new(payload.from_id.clone(), self.ctx_.clone(), None))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn to(&self) -> Option<Contact<T>> {
        debug!("message.to(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => {
                if !payload.to_id.is_empty() {
                    Some(Contact::new(payload.to_id.clone(), self.ctx_.clone(), None))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn room(&self) -> Option<Room<T>> {
        debug!("message.room(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => {
                if !payload.room_id.is_empty() {
                    Some(Room::new(payload.room_id.clone(), self.ctx_.clone(), None))
                } else {
                    None
                }
            }
            None => None,
        }
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

    // TODO: Analyze message text
    pub async fn mention_list(&mut self) -> Option<Vec<Contact<T>>> {
        debug!("message.mention_list(id = {})", self.id_);
        match &self.payload_ {
            Some(payload) => Some(self.ctx_.contact_load_batch(payload.mention_id_list.clone()).await),
            None => None,
        }
    }
}

impl<T> fmt::Debug for Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Message({})", self)
    }
}

impl<T> fmt::Display for Message<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from = match self.from() {
            Some(contact) => format!("From: {} ", contact),
            None => String::new(),
        };
        let to = match self.to() {
            Some(contact) => format!("To: {} ", contact),
            None => String::new(),
        };
        let room = match self.room() {
            Some(room) => format!("Room: {} ", room),
            None => String::new(),
        };
        let message_type = match self.message_type() {
            Some(message_type) => format!("Type: {:?} ", message_type),
            None => String::new(),
        };
        let text = if self.is_ready() && self.message_type().unwrap() == MessageType::Text {
            let text = self.text().unwrap().chars().collect::<Vec<_>>();
            let len = text.len().min(70);
            format!("Text: {} ", text[0..len].iter().collect::<String>())
        } else {
            String::new()
        };
        write!(fmt, "{}", [from, to, room, message_type, text].join(""))
    }
}
