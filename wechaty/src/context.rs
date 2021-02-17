use std::sync::{Arc, Mutex};

use wechaty_puppet::{Puppet, PuppetImpl};

use crate::WechatyPool;

#[derive(Clone)]
pub struct WechatyContext<T>
where
    T: PuppetImpl + Clone,
{
    puppet: Arc<Mutex<Puppet<T>>>,
    pool: Arc<Mutex<WechatyPool>>,
}

impl<T> WechatyContext<T>
where
    T: PuppetImpl + Clone,
{
    pub fn new(puppet_ptr: Arc<Mutex<Puppet<T>>>, pool_ptr: Arc<Mutex<WechatyPool>>) -> Self {
        Self {
            puppet: puppet_ptr,
            pool: pool_ptr,
        }
    }

    pub fn contact_find(&self) {}

    pub fn contact_find_all(&self) {}

    pub fn message_find(&self) {}

    pub fn message_find_all(&self) {}

    pub fn room_create(&self) {}

    pub fn room_find(&self) {}

    pub fn room_find_all(&self) {}

    pub fn friendship_add(&self) {}
}
