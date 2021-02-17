use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use log::{debug, error};
use wechaty_puppet::{ContactPayload, ContactQueryFilter, MessagePayload, Puppet, PuppetError, PuppetImpl};

use crate::Contact;

#[derive(Clone)]
pub struct WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    id_: Option<String>,
    puppet_: Puppet<T>,
    contacts_: Arc<Mutex<HashMap<String, ContactPayload>>>,
    messages_: Arc<Mutex<HashMap<String, MessagePayload>>>,
}

impl<T> WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub fn new(puppet: Puppet<T>) -> Self {
        Self {
            id_: None,
            puppet_: puppet,
            contacts_: Arc::new(Mutex::new(Default::default())),
            messages_: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn puppet(&self) -> Puppet<T> {
        self.puppet_.clone()
    }

    pub fn contacts(&self) -> MutexGuard<HashMap<String, ContactPayload>> {
        self.contacts_.lock().unwrap()
    }

    pub fn messages(&self) -> MutexGuard<HashMap<String, MessagePayload>> {
        self.messages_.lock().unwrap()
    }

    pub fn id(&self) -> Option<String> {
        self.id_.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.id_ = Some(id);
    }

    pub async fn contact_load(&self, contact_id: String) -> Result<Contact<T>, PuppetError> {
        let payload = {
            let contacts = self.contacts();
            if contacts.contains_key(&contact_id) {
                Some(contacts.get(&contact_id).unwrap().clone())
            } else {
                None
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

    pub fn contact_find(&self) {}

    pub async fn contact_find_all_by_string(&mut self, query_str: String) -> Result<Vec<Contact<T>>, PuppetError> {
        debug!("contact_find_all_by_string(query_str = {:?})", query_str);
        match self.puppet_.contact_search_by_string(query_str, None).await {
            Ok(contact_id_list) => {
                let mut contact_list = vec![];
                for contact_id in contact_id_list {
                    match self.contact_load(contact_id.clone()).await {
                        Ok(contact) => {
                            contact_list.push(contact);
                        }
                        Err(e) => {
                            error! {"Failed to load contact {}: {}", contact_id, e};
                        }
                    }
                }
                Ok(contact_list)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn contact_find_all(
        &mut self,
        query: Option<ContactQueryFilter>,
    ) -> Result<Vec<Contact<T>>, PuppetError> {
        debug!("contact_find_all(query = {:?})", query);
        let query = match query {
            Some(query) => query,
            None => ContactQueryFilter::default(),
        };
        match self.puppet_.contact_search(query, None).await {
            Ok(contact_id_list) => {
                let mut contact_list = vec![];
                for contact_id in contact_id_list {
                    match self.contact_load(contact_id).await {
                        Ok(contact) => {
                            contact_list.push(contact);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(contact_list)
            }
            Err(e) => Err(e),
        }
    }

    pub fn message_find(&self) {}

    pub fn message_find_all(&self) {}

    pub fn room_create(&self) {}

    pub fn room_find(&self) {}

    pub fn room_find_all(&self) {}

    pub fn friendship_add(&self) {}
}
