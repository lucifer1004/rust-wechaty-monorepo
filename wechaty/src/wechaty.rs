use actix::{Actor, Addr, Recipient};
use tokio::signal;
use wechaty_puppet::{Puppet, PuppetEvent, PuppetImpl};

use crate::{EventListener, EventListenerInner, WechatyContext};

type WechatyListener<T> = EventListenerInner<T>;

pub struct Wechaty<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    puppet: Puppet<T>,
    listener: WechatyListener<T>,
    addr: Addr<WechatyListener<T>>,
}

impl<T> Wechaty<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    pub fn new(puppet: Puppet<T>) -> Self {
        let listener = EventListenerInner::new("Wechaty".to_owned(), WechatyContext::new(puppet.clone()));
        let addr = listener.clone().start();
        Self { puppet, addr, listener }
    }

    pub async fn start(&self) {
        signal::ctrl_c()
            .await
            .expect("Failed to establish the listener for graceful exit");
    }
}

impl<T> EventListener<T> for Wechaty<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send,
{
    fn get_listener(&self) -> &EventListenerInner<T> {
        &self.listener
    }

    fn get_puppet(&self) -> Puppet<T> {
        self.puppet.clone()
    }

    fn get_addr(&self) -> Recipient<PuppetEvent> {
        self.addr.clone().recipient()
    }
}
