#![feature(async_closure)]

use wechaty::{
    wechaty_rt, DongPayload, EventListener, LoginPayload, MessagePayload, PuppetOptions, Wechaty, WechatyContext,
};
use wechaty_puppet_service::PuppetService;
use std::thread::sleep;
use std::time::Duration;

#[wechaty_rt::main]
async fn main() {
    env_logger::init();
    let token = env!("WECHATY_TOKEN");
    let mut bot = Wechaty::new(
        PuppetService::new(PuppetOptions {
            endpoint: None,
            timeout: None,
            token: Some(token.to_owned()),
        })
        .await
        .unwrap(),
    );

    bot.on_dong(async move |payload: DongPayload, ctx| println!("{}", payload.data))
        .on_login(async move |mut payload: LoginPayload<PuppetService>, ctx| {
            payload.sync().await;
            println!("{:?}", payload);
        })
        .on_message(async move |payload: MessagePayload, ctx| println!("{:?}", payload.message))
        .start()
        .await;
}
