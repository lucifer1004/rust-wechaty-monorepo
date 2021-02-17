use std::sync::{Arc, Mutex};

use log::{debug, error};
use wechaty_puppet::{ContactQueryFilter, Puppet, PuppetError, PuppetImpl};

use crate::{Contact, WechatyPool};

#[derive(Clone)]
pub struct WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    puppet: Puppet<T>,
    pool: Arc<Mutex<WechatyPool<T>>>,
}

impl<T> WechatyContext<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub fn new(puppet: Puppet<T>, pool_ptr: Arc<Mutex<WechatyPool<T>>>) -> Self {
        Self { puppet, pool: pool_ptr }
    }

    pub fn get_puppet(&self) -> Puppet<T> {
        self.puppet.clone()
    }

    pub async fn contact_load(&self, contact_id: String) -> Result<Contact<T>, PuppetError> {
        unimplemented!()
    }

    pub fn contact_find(&self) {}

    pub async fn contact_find_all_by_string(&mut self, query_str: String) -> Result<Vec<Contact<T>>, PuppetError> {
        debug!("contact_find_all_by_string(query_str = {:?})", query_str);
        match self.puppet.contact_search_by_string(query_str, None).await {
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

    pub async fn contact_find_all(&mut self, query: ContactQueryFilter) -> Result<Vec<Contact<T>>, PuppetError> {
        debug!("contact_find_all(query = {:?})", query);
        match self.puppet.contact_search(query, None).await {
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
