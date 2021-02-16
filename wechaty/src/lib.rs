mod wechaty;

pub use actix_rt as wechaty_rt;
pub use wechaty_puppet::PuppetOptions;

pub use wechaty::Wechaty;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
