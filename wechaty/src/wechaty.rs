use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, Recipient};
use tokio::signal;
use wechaty_puppet::{Puppet, PuppetEvent, PuppetImpl};

use crate::{EventListener, EventListenerInner, WechatyContext};

type WechatyListener<T> = EventListenerInner<WechatyContext<T>>;

pub struct Wechaty<T>
where
    T: PuppetImpl + Clone + 'static,
{
    puppet: Arc<Mutex<Puppet<T>>>,
    listener: WechatyListener<T>,
    addr: Addr<WechatyListener<T>>,
}

impl<T> Wechaty<T>
where
    T: PuppetImpl + Clone + 'static,
{
    pub fn new(puppet: Puppet<T>) -> Self {
        let puppet_ptr = Arc::new(Mutex::new(puppet));
        let listener = EventListenerInner::new("Wechaty".to_owned(), WechatyContext::new(puppet_ptr.clone()));
        let addr = listener.clone().start();
        Self {
            puppet: puppet_ptr,
            addr,
            listener,
        }
    }

    pub async fn start(&self) {
        signal::ctrl_c()
            .await
            .expect("Failed to establish the listener for graceful exit");
    }
}

impl<T> EventListener<T, WechatyContext<T>> for Wechaty<T>
where
    T: PuppetImpl + Clone,
{
    fn get_listener(&self) -> &EventListenerInner<WechatyContext<T>> {
        &self.listener
    }

    fn get_puppet(&self) -> &Arc<Mutex<Puppet<T>>> {
        &self.puppet
    }

    fn get_addr(&self) -> Recipient<PuppetEvent> {
        self.addr.clone().recipient()
    }
}
