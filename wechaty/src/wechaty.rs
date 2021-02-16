use wechaty_puppet::{Puppet, PuppetImpl};

pub struct Wechaty<T>
where
    T: PuppetImpl,
{
    puppet: Puppet<T>,
}

impl<T> Wechaty<T>
where
    T: PuppetImpl,
{
    pub fn new(puppet: Puppet<T>) -> Self {
        Self { puppet }
    }
}
