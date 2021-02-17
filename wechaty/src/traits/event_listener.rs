use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use actix::{Actor, ActorFuture, AtomicResponse, Context, Handler, Recipient, WrapFuture};
use log::{error, info};
use wechaty_puppet::{AsyncFnPtr, IntoAsyncFnPtr, Puppet, PuppetEvent, PuppetImpl, Subscribe};

use crate::{Contact, ContactSelf, DongPayload, LoginPayload, Message, MessagePayload};

pub trait EventListener<T, Context>
where
    T: PuppetImpl,
{
    fn get_listener(&self) -> &EventListenerInner<Context>;
    fn get_puppet(&self) -> &Arc<Mutex<Puppet<T>>>;
    fn get_addr(&self) -> Recipient<PuppetEvent>;
    fn get_name(&self) -> String {
        self.get_listener().name.clone()
    }

    fn on_event_with_handle<Payload>(
        &mut self,
        handler: AsyncFnPtr<Payload, Context, ()>,
        limit: Option<usize>,
        handlers: Rc<RefCell<Vec<(AsyncFnPtr<Payload, Context, ()>, usize)>>>,
        event_name: &'static str,
    ) -> (&mut Self, usize) {
        match self
            .get_puppet()
            .lock()
            .unwrap()
            .get_subscribe_addr()
            .do_send(Subscribe {
                addr: self.get_addr(),
                name: self.get_name(),
                event_name,
            }) {
            Err(e) => {
                error!("{} failed to subscribe to event {}: {}", self.get_name(), event_name, e);
            }
            Ok(_) => {}
        }
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
        F: IntoAsyncFnPtr<DongPayload, Context, ()>,
    {
        self.on_dong_with_handle(handler, None);
        self
    }

    fn on_dong_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<DongPayload, Context, ()>,
    {
        let dong_handlers = self.get_listener().dong_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, dong_handlers, "dong")
    }

    fn on_login<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<LoginPayload, Context, ()>,
    {
        self.on_login_with_handle(handler, None);
        self
    }

    fn on_login_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<LoginPayload, Context, ()>,
    {
        let login_handlers = self.get_listener().login_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, login_handlers, "login")
    }

    fn on_message<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<MessagePayload, Context, ()>,
    {
        self.on_message_with_handle(handler, None);
        self
    }

    fn on_message_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<MessagePayload, Context, ()>,
    {
        let message_handlers = self.get_listener().message_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, message_handlers, "message")
    }
}

type HandlersPtr<Payload, Context> = Rc<RefCell<Vec<(AsyncFnPtr<Payload, Context, ()>, usize)>>>;

#[derive(Clone)]
pub struct EventListenerInner<Context> {
    name: String,
    ctx: Context,
    dong_handlers: HandlersPtr<DongPayload, Context>,
    login_handlers: HandlersPtr<LoginPayload, Context>,
    message_handlers: HandlersPtr<MessagePayload, Context>,
}

impl<T> Actor for EventListenerInner<T>
where
    T: 'static + Unpin,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("{} started", self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("{} stopped", self.name);
    }
}

impl<T> Handler<PuppetEvent> for EventListenerInner<T>
where
    T: 'static + Clone + Unpin,
{
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: PuppetEvent, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PuppetEvent::Dong(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(|_, this, _| {
                    this.trigger_handlers(payload, this.dong_handlers.clone())
                        .into_actor(this)
                })))
            }
            PuppetEvent::Login(payload) => {
                let payload = LoginPayload {
                    contact_self: ContactSelf {},
                };

                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(|_, this, _| {
                    this.trigger_handlers(payload, this.login_handlers.clone())
                        .into_actor(this)
                })))
            }
            PuppetEvent::Message(payload) => {
                let payload = MessagePayload { message: Message {} };

                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(|_, this, _| {
                    this.trigger_handlers(payload, this.message_handlers.clone())
                        .into_actor(this)
                })))
            }
            _ => AtomicResponse::new(Box::pin(async {}.into_actor(self))),
        }
    }
}

impl<Context> EventListenerInner<Context>
where
    Context: Clone + 'static,
{
    pub(crate) fn new(name: String, ctx: Context) -> Self {
        Self {
            name,
            ctx,
            dong_handlers: Rc::new(RefCell::new(vec![])),
            login_handlers: Rc::new(RefCell::new(vec![])),
            message_handlers: Rc::new(RefCell::new(vec![])),
        }
    }

    fn trigger_handlers<Payload: Clone + 'static>(
        &mut self,
        payload: Payload,
        handlers: Rc<RefCell<Vec<(AsyncFnPtr<Payload, Context, ()>, usize)>>>,
    ) -> impl Future<Output = ()> + 'static {
        let len = handlers.borrow_mut().len();
        let ctx = self.ctx.clone();
        async move {
            for i in 0..len {
                let mut handler = &mut handlers.borrow_mut()[i];
                if handler.1 > 0 {
                    handler.0.run(payload.clone(), ctx.clone()).await;
                    handler.1 -= 1;
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wechaty_puppet_service::PuppetService;
//
//     struct MockListener {
//         addr: Addr<EventListenerInner>,
//         listener: EventListenerInner,
//     }
//
//     impl<T> EventListener<T> for MockListener
//         where T: PuppetImpl
//     {
//         fn get_listener(&self) -> &EventListenerInner {
//             &self.listener
//         }
//
//         fn get_puppet(&self) -> &Puppet<T> {
//             unimplemented!()
//         }
//
//         fn get_addr(&self) -> Recipient<PuppetEvent> {
//             unimplemented!()
//         }
//     }
//
//     impl MockListener {
//         fn new() -> Self {
//             let listener = EventListenerInner::new("MockListener".to_owned());
//             Self {
//                 addr: listener.clone().start(),
//                 listener,
//             }
//         }
//
//         async fn dong(&self) {
//             match self
//                 .addr
//                 .send(PuppetEvent::Dong(EventDongPayload {
//                     data: "dong".to_string(),
//                 }))
//                 .await
//             {
//                 Err(e) => panic!("{}", e),
//                 _ => (),
//             }
//         }
//     }
//
//     async fn handle_dong(payload: EventDongPayload) {
//         println!("Got {}!", payload.data);
//     }
//
//     #[actix_rt::test]
//     async fn test_mock_listener() {
//         let mut mock_listener = MockListener::new();
//         mock_listener.on_dong(handle_dong);
//         mock_listener.dong().await;
//     }
// }
