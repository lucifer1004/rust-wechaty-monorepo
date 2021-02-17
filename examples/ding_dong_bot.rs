#![feature(async_closure)]
use wechaty::{
    wechaty_rt, DongPayload, EventListener, LoginPayload, MessagePayload, PuppetOptions, Wechaty, WechatyContext,
};
use wechaty_puppet_service::PuppetService;

#[wechaty_rt::main]
async fn main() {
    env_logger::init();
    let endpoint = env!("WECHATY_ENDPOINT");
    let token = env!("WECHATY_TOKEN");
    let mut bot = Wechaty::new(
        PuppetService::new(PuppetOptions {
            endpoint: Some(endpoint.to_owned()),
            timeout: None,
            token: Some(token.to_owned()),
        })
        .await
        .unwrap(),
    );

    bot.on_dong(async move |payload: DongPayload, ctx| println!("{}", payload.data))
        .on_login(
            async move |mut payload: LoginPayload<PuppetService>, mut ctx: WechatyContext<PuppetService>| {
                println!("{:?}", ctx.contact_find_all(None).await.unwrap());
            },
        )
        .on_message(async move |payload: MessagePayload, ctx| println!("{:?}", payload.message))
        .start()
        .await;
}
