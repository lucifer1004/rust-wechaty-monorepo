use std::collections::HashMap;

use wechaty_puppet::PuppetImpl;

use crate::{Contact, Message};

pub struct WechatyPool<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    contacts: HashMap<String, Contact<T>>,
    messages: HashMap<String, Message>,
}

impl<T> WechatyPool<T>
where
    T: 'static + PuppetImpl + Clone + Unpin,
{
    pub fn new() -> Self {
        Self {
            contacts: Default::default(),
            messages: Default::default(),
        }
    }

    pub fn contact_load(&self, contact_id: String) {}
}
