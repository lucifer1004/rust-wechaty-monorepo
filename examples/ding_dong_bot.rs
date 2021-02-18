#![feature(async_closure)]
use wechaty::{wechaty_rt, DongPayload, EventListener, LoginPayload, MessagePayload, PuppetOptions, ScanPayload, Wechaty, WechatyContext, LogoutPayload};
use wechaty_puppet::{UrlLinkPayload, MessageType};
use wechaty_puppet_service::PuppetService;

#[wechaty_rt::main]
async fn main() {
    env_logger::init();
    let endpoint = env!("WECHATY_ENDPOINT");
    let mut bot = Wechaty::new(
        PuppetService::new(PuppetOptions {
            endpoint: Some(endpoint.to_owned()),
            timeout: None,
            token: None,
        })
        .await
        .unwrap(),
    );

    bot.on_scan(async move |payload: ScanPayload, ctx| {
        if let Some(qrcode) = payload.qrcode {
            println!(
                "Visit {} to log in",
                format!("https://wechaty.js.org/qrcode/{}", qrcode)
            );
        }
    })
    .on_login(async move |payload: LoginPayload<PuppetService>, ctx| {
        println!("User {} has logged in", payload.contact);
    })
    .on_logout(async move |payload: LogoutPayload<PuppetService>, ctx| {
        println!("User {} has logged out", payload.contact);
    })
    .on_message(async move |payload: MessagePayload<PuppetService>, ctx: WechatyContext<PuppetService>| {
        let mut message = payload.message;
        println!("Got message: {}", message);
        if message.is_self() {
            println!("Message discarded because its outgoing");
            return;
        }
        if let Some(message_type) = message.message_type() {
            if message.text().unwrap() == "ding" {
                message.from().unwrap().send_text("dong".to_owned()).await;
                println!("REPLY: dong");
            } else {
                println!("Message discarded because it does not match ding");
            }
        } else {
            println!("Message discarded because it is not a text");
        }
    })
    .start()
    .await;
}
