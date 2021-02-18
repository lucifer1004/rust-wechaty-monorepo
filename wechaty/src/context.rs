use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::future::join_all;
use futures::StreamExt;
use log::{debug, error};
use wechaty_puppet::{ContactPayload, ContactQueryFilter, MessagePayload, Puppet, PuppetImpl};

use crate::{Contact, Message, Room, WechatyError};

#[derive(Clone)]
pub struct WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    id_: Option<String>,
    puppet_: Puppet<T>,
    contacts_: Arc<Mutex<HashMap<String, ContactPayload>>>,
    messages_: Arc<Mutex<HashMap<String, MessagePayload>>>,
}

impl<T> WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    pub(crate) fn new(puppet: Puppet<T>) -> Self {
        Self {
            id_: None,
            puppet_: puppet,
            contacts_: Arc::new(Mutex::new(Default::default())),
            messages_: Arc::new(Mutex::new(Default::default())),
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

    pub(crate) fn id(&self) -> Option<String> {
        self.id_.clone()
    }

    pub(crate) fn set_id(&mut self, id: String) {
        self.id_ = Some(id);
    }

    pub(crate) async fn contact_load(&self, contact_id: String) -> Result<Contact<T>, WechatyError> {
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
                    return Err(e);
                }
                Ok(contact)
            }
        }
    }

    pub async fn contact_find(&self) {
        unimplemented!()
    }

    /// Batch load contacts with a default batch size of 16.
    ///
    /// Reference: [Batch execution of futures in the tokio runtime](https://users.rust-lang.org/t/batch-execution-of-futures-in-the-tokio-runtime-or-max-number-of-active-futures-at-a-time/47659).
    ///
    /// Note the API changes: `tokio::stream::iter` is now temporarily `tokio_stream::iter`, according to
    /// [tokio's tutorial](https://tokio.rs/tokio/tutorial/streams), it will be moved back to the `tokio`
    /// crate when the `Stream` trait is stable.
    async fn contact_load_batch(&mut self, contact_id_list: Vec<String>) -> Vec<Contact<T>> {
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

    pub async fn contact_find_all_by_string(&mut self, query_str: String) -> Result<Vec<Contact<T>>, WechatyError> {
        debug!("contact_find_all_by_string(query_str = {:?})", query_str);
        match self.puppet_.contact_search_by_string(query_str, None).await {
            Ok(contact_id_list) => Ok(self.contact_load_batch(contact_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }

    pub async fn contact_find_all(
        &mut self,
        query: Option<ContactQueryFilter>,
    ) -> Result<Vec<Contact<T>>, WechatyError> {
        debug!("contact_find_all(query = {:?})", query);
        let query = match query {
            Some(query) => query,
            None => ContactQueryFilter::default(),
        };
        match self.puppet_.contact_search(query, None).await {
            Ok(contact_id_list) => Ok(self.contact_load_batch(contact_id_list).await),
            Err(e) => Err(WechatyError::from(e)),
        }
    }

    pub async fn message_load(&self, message_id: String) -> Result<Message<T>, WechatyError> {
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

    pub async fn message_find(&self) {
        unimplemented!()
    }

    pub async fn message_find_all(&self) {
        unimplemented!()
    }

    pub async fn room_load(&self, room_id: String) -> Result<Room<T>, WechatyError> {
        unimplemented!()
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
}
