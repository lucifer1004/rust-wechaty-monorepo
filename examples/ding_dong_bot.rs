#![feature(async_closure)]
use wechaty::{
    wechaty_rt, DongPayload, EventListener, LoginPayload, MessagePayload, PuppetOptions, Wechaty, WechatyContext,
};
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

    bot.on_dong(async move |payload: DongPayload, ctx| println!("{}", payload.data))
        .on_login(
            async move |mut payload: LoginPayload<PuppetService>, mut ctx: WechatyContext<PuppetService>| {
                println!("Contact list: {:?}", ctx.contact_find_all(None).await.unwrap());
            },
        )
        .on_message(async move |payload: MessagePayload<PuppetService>, ctx| {
            println!("{}", payload.message);
        })
        .start()
        .await;
}
