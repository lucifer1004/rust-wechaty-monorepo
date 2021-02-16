use wechaty::{wechaty_rt, PuppetOptions, Wechaty};
use wechaty_puppet_service::PuppetService;

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
}
