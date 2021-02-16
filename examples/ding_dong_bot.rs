use wechaty::{wechaty_rt, EventListener, PuppetOptions, Wechaty};
use wechaty_puppet::EventDongPayload;
use wechaty_puppet_service::PuppetService;

async fn handle_dong(payload: EventDongPayload) {
    println!("{}", payload.data);
}

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
    bot.on_dong(handle_dong);
}
