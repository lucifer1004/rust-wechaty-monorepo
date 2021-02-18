#![feature(async_closure)]
use std::env;

use wechaty::{
    wechaty_rt, EventListener, LoginPayload, LogoutPayload, MessagePayload, MessageType, PuppetOptions, ScanPayload,
    Wechaty, WechatyContext,
};
use wechaty_puppet_service::PuppetService;

#[wechaty_rt::main]
async fn main() {
    env_logger::init();
    let options = PuppetOptions {
        endpoint: match env::var("WECHATY_ENDPOINT") {
            Ok(endpoint) => Some(endpoint),
            Err(_) => None,
        },
        timeout: None,
        token: match env::var("WECHATY_TOKEN") {
            Ok(endpoint) => Some(endpoint),
            Err(_) => None,
        },
    };
    let mut bot = Wechaty::new(PuppetService::new(options).await.unwrap());

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
    .on_message(
        async move |payload: MessagePayload<PuppetService>, ctx| {
            let message = payload.message;
            println!("Got message: {}", message);
            if message.is_self() {
                println!("Message discarded because its outgoing");
                return;
            }
            if let Some(message_type) = message.message_type() {
                if message_type != MessageType::Text || message.text().unwrap() != "ding" {
                    println!("Message discarded because it does not match ding");
                } else {
                    if let Err(e) = message.from().unwrap().send_text("dong".to_owned()).await {
                        println!("Failed to send message");
                    } else {
                        println!("REPLY: dong");
                    }
                }
            } else {
                println!("Message discarded because it is not a text");
            }
        },
    )
    .start()
    .await;
}
