use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::StreamExt;
use log::{debug, error};
use wechaty_puppet::{
    ContactPayload, ContactQueryFilter, MessagePayload, MessageQueryFilter, Puppet, PuppetImpl, RoomPayload,
};

use crate::{Contact, IntoContact, Message, Room, WechatyError};

#[derive(Clone)]
pub struct WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    id_: Option<String>,
    puppet_: Puppet<T>,
    contacts_: Arc<Mutex<HashMap<String, ContactPayload>>>,
    messages_: Arc<Mutex<HashMap<String, MessagePayload>>>,
    rooms_: Arc<Mutex<HashMap<String, RoomPayload>>>,
}

impl<T> WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub(crate) fn new(puppet: Puppet<T>) -> Self {
        Self {
            id_: None,
            puppet_: puppet,
            contacts_: Arc::new(Mutex::new(Default::default())),
            messages_: Arc::new(Mutex::new(Default::default())),
            rooms_: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub(crate) fn puppet(&self) -> Puppet<T> {
        self.puppet_.clone()
    }

    pub(crate) fn contacts(&self) -> MutexGuard<HashMap<String, ContactPayload>> {
        self.contacts_.lock().unwrap()
    }

    pub(crate) fn messages(&self) -> MutexGuard<HashMap<String, MessagePayload>> {
        self.messages_.lock().unwrap()
    }

    pub(crate) fn rooms(&self) -> MutexGuard<HashMap<String, RoomPayload>> {
        self.rooms_.lock().unwrap()
    }

    pub(crate) fn id(&self) -> Option<String> {
        self.id_.clone()
    }

    pub(crate) fn set_id(&mut self, id: String) {
        self.id_ = Some(id);
    }

    pub(crate) fn clear_id(&mut self) {
        self.id_ = None;
    }

    pub(crate) fn is_logged_in(&self) -> bool {
        self.id_.is_some()
    }

    /// Load a contact.
    ///
    /// Use contact store first, if the contact cannot be found in the local store,
    /// try to fetch from the puppet instead.
    pub(crate) async fn contact_load(&self, contact_id: String) -> Result<Contact<T>, WechatyError> {
        debug!("contact_load(query = {})", contact_id);

        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }

        let payload = {
            match self.contacts().get(&contact_id) {
                Some(payload) => Some(payload.clone()),
                None => None,
            }
        };
        match payload {
            Some(payload) => Ok(Contact::new(contact_id.clone(), self.clone(), Some(payload))),
            None => {
                let mut contact = Contact::new(contact_id.clone(), self.clone(), None);
                if let Err(e) = contact.sync().await {
                    error!("Failed to get payload of contact {}", contact_id);
                    return Err(e);
                }
                Ok(contact)
            }
        }
    }

    /// Batch load contacts with a default batch size of 16.
    ///
    /// Reference: [Batch execution of futures in the tokio runtime](https://users.rust-lang.org/t/batch-execution-of-futures-in-the-tokio-runtime-or-max-number-of-active-futures-at-a-time/47659).
    ///
    /// Note the API change: `tokio::stream::iter` is now temporarily `tokio_stream::iter`, according to
    /// [tokio's tutorial](https://tokio.rs/tokio/tutorial/streams), it will be moved back to the `tokio`
    /// crate when the `Stream` trait is stable.
    pub(crate) async fn contact_load_batch(&self, contact_id_list: Vec<String>) -> Vec<Contact<T>> {
        debug!("contact_load_batch(contact_id_list = {:?})", contact_id_list);
        let mut contact_list = vec![];
        let mut stream = tokio_stream::iter(contact_id_list)
            .map(|contact_id| self.contact_load(contact_id))
            .buffer_unordered(16);
        while let Some(result) = stream.next().await {
            if let Ok(contact) = result {
                contact_list.push(contact);
            }
        }
        contact_list
    }

    /// Find the first contact that matches the query
    pub async fn contact_find(&self, query: ContactQueryFilter) -> Result<Option<Contact<T>>, WechatyError> {
        debug!("contact_find(query = {:?})", query);
        match self.contact_find_all(Some(query)).await {
            Ok(contact_list) => {
                if contact_list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(contact_list[0].clone()))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Find the first contact that matches the query string
    pub async fn contact_find_by_string(&self, query_str: String) -> Result<Option<Contact<T>>, WechatyError> {
        debug!("contact_find_by_string(query_str = {:?})", query_str);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        match self.contact_find_all_by_string(query_str).await {
            Ok(contact_list) => {
                if contact_list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(contact_list[0].clone()))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Find all contacts that match the query
    pub async fn contact_find_all(&self, query: Option<ContactQueryFilter>) -> Result<Vec<Contact<T>>, WechatyError> {
        debug!("contact_find_all(query = {:?})", query);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        let query = match query {
            Some(query) => query,
            None => ContactQueryFilter::default(),
        };
        match self.puppet().contact_search(query, None).await {
            Ok(contact_id_list) => Ok(self.contact_load_batch(contact_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }

    /// Find all contacts that match the query string
    pub async fn contact_find_all_by_string(&self, query_str: String) -> Result<Vec<Contact<T>>, WechatyError> {
        debug!("contact_find_all_by_string(query_str = {:?})", query_str);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        match self.puppet().contact_search_by_string(query_str, None).await {
            Ok(contact_id_list) => Ok(self.contact_load_batch(contact_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }

    /// Load a message.
    ///
    /// Use message store first, if the message cannot be found in the local store,
    /// try to fetch from the puppet instead.
    pub(crate) async fn message_load(&self, message_id: String) -> Result<Message<T>, WechatyError> {
        debug!("message_load(query = {})", message_id);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        let payload = {
            match self.messages().get(&message_id) {
                Some(payload) => Some(payload.clone()),
                None => None,
            }
        };
        match payload {
            Some(payload) => Ok(Message::new(message_id.clone(), self.clone(), Some(payload))),
            None => {
                let mut message = Message::new(message_id.clone(), self.clone(), None);
                if let Err(e) = message.ready().await {
                    return Err(e);
                }
                Ok(message)
            }
        }
    }

    /// Batch load messages with a default batch size of 16.
    pub(crate) async fn message_load_batch(&self, message_id_list: Vec<String>) -> Vec<Message<T>> {
        debug!("message_load_batch(message_id_list = {:?})", message_id_list);
        let mut message_list = vec![];
        let mut stream = tokio_stream::iter(message_id_list)
            .map(|message_id| self.message_load(message_id))
            .buffer_unordered(16);
        while let Some(result) = stream.next().await {
            if let Ok(message) = result {
                message_list.push(message);
            }
        }
        message_list
    }

    /// Find the first message that matches the query
    pub async fn message_find(&self, query: MessageQueryFilter) -> Result<Option<Message<T>>, WechatyError> {
        debug!("message_find(query = {:?})", query);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        match self.message_find_all(query).await {
            Ok(message_list) => {
                if message_list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(message_list[0].clone()))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Find all messages that match the query
    pub async fn message_find_all(&self, query: MessageQueryFilter) -> Result<Vec<Message<T>>, WechatyError> {
        debug!("message_find_all(query = {:?}", query);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        match self.puppet().message_search(query).await {
            Ok(message_id_list) => Ok(self.message_load_batch(message_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }

    pub(crate) async fn room_load(&self, room_id: String) -> Result<Room<T>, WechatyError> {
        debug!("room_load(room_id = {})", room_id);
        if !self.is_logged_in() {
            return Err(WechatyError::NotLoggedIn);
        }
        let payload = {
            match self.rooms().get(&room_id) {
                Some(payload) => Some(payload.clone()),
                None => None,
            }
        };
        match payload {
            Some(payload) => Ok(Room::new(room_id.clone(), self.clone(), Some(payload))),
            None => {
                let mut room = Room::new(room_id.clone(), self.clone(), None);
                if let Err(e) = room.sync().await {
                    return Err(e);
                }
                Ok(room)
            }
        }
    }

    pub async fn room_create(&self) {
        unimplemented!()
    }

    pub async fn room_find(&self) {
        unimplemented!()
    }

    pub async fn room_find_all(&self) {
        unimplemented!()
    }

    pub async fn friendship_add(&self) {
        unimplemented!()
    }

    pub async fn logout(&self) -> Result<(), WechatyError> {
        match self.puppet().logout().await {
            Ok(_) => Ok(()),
            Err(e) => Err(WechatyError::from(e)),
        }
    }
}
