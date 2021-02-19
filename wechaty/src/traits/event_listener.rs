use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use actix::{
    Actor, ActorContext, ActorFuture, AtomicResponse, Context, Handler, Message as ActorMessage, Recipient, WrapFuture,
};
use log::{error, info};
use wechaty_puppet::{AsyncFnPtr, IntoAsyncFnPtr, Puppet, PuppetEvent, PuppetImpl, Subscribe};

use crate::user::contact_self::ContactSelf;
use crate::{
    Contact, DongPayload, IntoContact, LoginPayload, LogoutPayload, Message, MessagePayload, ScanPayload,
    WechatyContext,
};

#[derive(ActorMessage)]
#[rtype("()")]
pub(crate) enum Command {
    Stop,
}

pub trait EventListener<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    fn get_listener(&self) -> &EventListenerInner<T>;
    fn get_puppet(&self) -> Puppet<T>;
    fn get_addr(&self) -> Recipient<PuppetEvent>;
    fn get_name(&self) -> String {
        self.get_listener().name.clone()
    }

    fn on_event_with_handle<Payload>(
        &mut self,
        handler: AsyncFnPtr<Payload, WechatyContext<T>, ()>,
        limit: Option<usize>,
        handlers: Rc<RefCell<Vec<(AsyncFnPtr<Payload, WechatyContext<T>, ()>, usize)>>>,
        event_name: &'static str,
    ) -> (&mut Self, usize) {
        match self.get_puppet().get_subscribe_addr().do_send(Subscribe {
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
        F: IntoAsyncFnPtr<DongPayload, WechatyContext<T>, ()>,
    {
        self.on_dong_with_handle(handler, None);
        self
    }

    fn on_dong_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<DongPayload, WechatyContext<T>, ()>,
    {
        let dong_handlers = self.get_listener().dong_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, dong_handlers, "dong")
    }

    fn on_login<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<LoginPayload<T>, WechatyContext<T>, ()>,
    {
        self.on_login_with_handle(handler, None);
        self
    }

    fn on_login_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<LoginPayload<T>, WechatyContext<T>, ()>,
    {
        let login_handlers = self.get_listener().login_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, login_handlers, "login")
    }

    fn on_logout<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<LogoutPayload<T>, WechatyContext<T>, ()>,
    {
        self.on_logout_with_handle(handler, None);
        self
    }

    fn on_logout_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<LogoutPayload<T>, WechatyContext<T>, ()>,
    {
        let logout_handlers = self.get_listener().logout_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, logout_handlers, "logout")
    }

    fn on_message<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<MessagePayload<T>, WechatyContext<T>, ()>,
    {
        self.on_message_with_handle(handler, None);
        self
    }

    fn on_message_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<MessagePayload<T>, WechatyContext<T>, ()>,
    {
        let message_handlers = self.get_listener().message_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, message_handlers, "message")
    }

    fn on_scan<F>(&mut self, handler: F) -> &mut Self
    where
        F: IntoAsyncFnPtr<ScanPayload, WechatyContext<T>, ()>,
    {
        self.on_scan_with_handle(handler, None);
        self
    }

    fn on_scan_with_handle<F>(&mut self, handler: F, limit: Option<usize>) -> (&mut Self, usize)
    where
        F: IntoAsyncFnPtr<ScanPayload, WechatyContext<T>, ()>,
    {
        let scan_handlers = self.get_listener().scan_handlers.clone();
        self.on_event_with_handle(handler.into(), limit, scan_handlers, "scan")
    }
}

type HandlersPtr<T, Payload> = Rc<RefCell<Vec<(AsyncFnPtr<Payload, WechatyContext<T>, ()>, usize)>>>;

#[derive(Clone)]
pub struct EventListenerInner<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    name: String,
    ctx: WechatyContext<T>,
    dong_handlers: HandlersPtr<T, DongPayload>,
    login_handlers: HandlersPtr<T, LoginPayload<T>>,
    logout_handlers: HandlersPtr<T, LogoutPayload<T>>,
    message_handlers: HandlersPtr<T, MessagePayload<T>>,
    scan_handlers: HandlersPtr<T, ScanPayload>,
}

impl<T> Actor for EventListenerInner<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
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
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: PuppetEvent, _ctx: &mut Context<Self>) -> Self::Result {
        info!("{} receives puppet event: {:?}", self.name.clone(), msg);
        let ctx = self.ctx.clone();
        match msg {
            PuppetEvent::Dong(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(move |_, this, _| {
                    EventListenerInner::<T>::trigger_handlers(ctx, payload, this.dong_handlers.clone()).into_actor(this)
                })))
            }
            PuppetEvent::Login(payload) => {
                self.ctx.set_id(payload.contact_id.clone());
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(move |_, this, _| {
                    this.trigger_login_handlers(payload.contact_id.clone()).into_actor(this)
                })))
            }
            PuppetEvent::Logout(payload) => {
                self.ctx.clear_id();
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(move |_, this, _| {
                    this.trigger_logout_handlers(payload.contact_id.clone())
                        .into_actor(this)
                })))
            }
            PuppetEvent::Message(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(move |_, this, _| {
                    this.trigger_message_handlers(payload.message_id).into_actor(this)
                })))
            }
            PuppetEvent::Scan(payload) => {
                AtomicResponse::new(Box::pin(async {}.into_actor(self).then(move |_, this, _| {
                    EventListenerInner::<T>::trigger_handlers(
                        ctx,
                        ScanPayload::new(payload.qrcode, payload.status),
                        this.scan_handlers.clone(),
                    )
                    .into_actor(this)
                })))
            }
            _ => AtomicResponse::new(Box::pin(async {}.into_actor(self))),
        }
    }
}

impl<T> Handler<Command> for EventListenerInner<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    type Result = ();

    fn handle(&mut self, msg: Command, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Command::Stop => ctx.stop(),
        }
    }
}

impl<T> EventListenerInner<T>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub(crate) fn new(name: String, ctx: WechatyContext<T>) -> Self {
        Self {
            name,
            ctx,
            dong_handlers: Rc::new(RefCell::new(vec![])),
            login_handlers: Rc::new(RefCell::new(vec![])),
            logout_handlers: Rc::new(RefCell::new(vec![])),
            message_handlers: Rc::new(RefCell::new(vec![])),
            scan_handlers: Rc::new(RefCell::new(vec![])),
        }
    }

    async fn trigger_handlers<Payload: Clone + 'static>(
        ctx: WechatyContext<T>,
        payload: Payload,
        handlers: HandlersPtr<T, Payload>,
    ) where
        T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
    {
        let len = handlers.borrow_mut().len();
        for i in 0..len {
            let mut handler = &mut handlers.borrow_mut()[i];
            if handler.1 > 0 {
                handler.0.run(payload.clone(), ctx.clone()).await;
                handler.1 -= 1;
            }
        }
    }

    fn trigger_login_handlers(&mut self, contact_id: String) -> impl Future<Output = ()> + 'static {
        let mut contact = ContactSelf::new(contact_id.clone(), self.ctx.clone(), None);
        let ctx = self.ctx.clone();
        let handlers = self.login_handlers.clone();
        async move {
            contact.sync().await;
            EventListenerInner::<T>::trigger_handlers(ctx, LoginPayload { contact }, handlers).await
        }
    }

    fn trigger_logout_handlers(&mut self, contact_id: String) -> impl Future<Output = ()> + 'static {
        let mut contact = ContactSelf::new(contact_id.clone(), self.ctx.clone(), None);
        let ctx = self.ctx.clone();
        let handlers = self.logout_handlers.clone();
        async move {
            contact.ready(false).await;
            EventListenerInner::<T>::trigger_handlers(ctx, LogoutPayload { contact }, handlers).await
        }
    }

    fn trigger_message_handlers(&mut self, message_id: String) -> impl Future<Output = ()> + 'static {
        let mut message = Message::new(message_id.clone(), self.ctx.clone(), None);
        let ctx = self.ctx.clone();
        let handlers = self.message_handlers.clone();
        async move {
            message.ready().await;
            EventListenerInner::<T>::trigger_handlers(ctx, MessagePayload { message }, handlers).await
        }
    }
}
