use wechaty_puppet::PuppetImpl;

use crate::WechatyContext;

#[derive(Clone)]
pub struct Entity<T, Payload>
where
    T: 'static + PuppetImpl + Clone + Unpin + Send + Sync,
{
    pub(crate) ctx: WechatyContext<T>,
    pub(crate) id_: String,
    pub(crate) payload_: Option<Payload>,
}
