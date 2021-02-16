use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use actix::{Actor, ActorFuture, Addr, AtomicResponse, Context, Handler, Message as ActorMessage, WrapFuture};
use log::{debug, info};
use wechaty_puppet::{AsyncFnPtr, EventDongPayload, EventMessagePayload, IntoAsyncFnPtr, PuppetEvent};

pub trait EventListener {
    fn get_listener(&mut self) -> &mut EventListenerInner;

    fn on_event_with_handle<T>(
        &mut self,
        handler: AsyncFnPtr<T, ()>,
        limit: Option<usize>,
        handlers: Rc<RefCell<Vec<(AsyncFnPtr<T, ()>, usize)>>>,
    ) -> (&mut Self, usize) {
        let counter = handlers.borrow().len();
        let limit = match limit {
            Some(limit) => limit,
            None => usize::MAX,
        };
        handlers.borrow_mut().push((handler, limit));
        (self, counter)
    }

    fn on_dong<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<EventDongPayload, ()>,
    {
        self.on_dong_with_handle(handler, None);
        self
    }

    fn on_dong_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<EventDongPayload, ()>,
    {
        let dong_handlers = self.get_listener().dong_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, dong_handlers)
    }

    fn on_message<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<EventMessagePayload, ()>,
    {
        self.on_message_with_handle(handler, None);
        self
    }

    fn on_message_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<EventMessagePayload, ()>,
    {
        let message_handlers = self.get_listener().message_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, message_handlers)
    }
}

#[derive(Clone)]
pub struct EventListenerInner {
    name: String,
    dong_handlers: Rc<RefCell<Vec<(AsyncFnPtr<EventDongPayload, ()>, usize)>>>,
    message_handlers: Rc<RefCell<Vec<(AsyncFnPtr<EventMessagePayload, ()>, usize)>>>,
}

impl Actor for EventListenerInner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("{} started", self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("{} stopped", self.name);
    }
}

impl Handler<PuppetEvent> for EventListenerInner {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: PuppetEvent, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PuppetEvent::Dong(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(|_, this, _| {
                    this.trigger_handlers(payload, this.dong_handlers.clone())
                        .into_actor(this)
                })))
            }
            PuppetEvent::Message(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(|_, this, _| {
                    this.trigger_handlers(payload, this.message_handlers.clone())
                        .into_actor(this)
                })))
            }
            _ => AtomicResponse::new(Box::pin(async {}.into_actor(self))),
        }
    }
}

impl EventListenerInner {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            dong_handlers: Rc::new(RefCell::new(vec![])),
            message_handlers: Rc::new(RefCell::new(vec![])),
        }
    }

    fn trigger_handlers<T: Clone + 'static>(
        &mut self,
        payload: T,
        handlers: Rc<RefCell<Vec<(AsyncFnPtr<T, ()>, usize)>>>,
    ) -> impl Future<Output = ()> + 'static {
        let len = handlers.borrow_mut().len();
        async move {
            for i in 0..len {
                let mut handler = &mut handlers.borrow_mut()[i];
                if handler.1 > 0 {
                    handler.0.run(payload.clone()).await;
                    handler.1 -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockListener {
        addr: Addr<EventListenerInner>,
        listener: EventListenerInner,
    }

    impl EventListener for MockListener {
        fn get_listener(&mut self) -> &mut EventListenerInner {
            &mut self.listener
        }
    }

    impl MockListener {
        fn new() -> Self {
            let listener = EventListenerInner::new("MockListener".to_owned());
            Self {
                addr: listener.clone().start(),
                listener,
            }
        }

        async fn dong(&self) {
            match self
                .addr
                .send(PuppetEvent::Dong(EventDongPayload {
                    data: "dong".to_string(),
                }))
                .await
            {
                Err(e) => panic!("{}", e),
                _ => (),
            }
        }
    }

    async fn handle_dong(payload: EventDongPayload) {
        println!("Got {}!", payload.data);
    }

    #[actix_rt::test]
    async fn test_mock_listener() {
        let mut mock_listener = MockListener::new();
        mock_listener.on_dong(handle_dong);
        mock_listener.dong().await;
    }
}
