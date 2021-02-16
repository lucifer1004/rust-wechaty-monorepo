use actix::{Actor, Addr, Context};
use log::info;
use wechaty_puppet::{Puppet, PuppetImpl};

use crate::traits::event_listener::{EventListener, EventListenerInner};

pub struct Wechaty<T>
where
    T: PuppetImpl,
{
    puppet: Puppet<T>,
    listener: EventListenerInner,
    addr: Addr<EventListenerInner>,
}

impl<T> Wechaty<T>
where
    T: PuppetImpl,
{
    pub fn new(puppet: Puppet<T>) -> Self {
        let listener = EventListenerInner::new("Wechaty".to_owned());
        Self {
            puppet,
            addr: listener.clone().start(),
            listener,
        }
    }
}

impl<T> EventListener for Wechaty<T>
where
    T: PuppetImpl,
{
    fn get_listener(&mut self) -> &mut EventListenerInner {
        &mut self.listener
    }
}
