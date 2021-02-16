#[macro_use]
extern crate num_derive;

pub mod error;
pub mod events;
pub mod puppet;
pub mod schemas;
pub mod types;

pub use events::PuppetEvent;
pub use puppet::{Puppet, PuppetImpl};
pub use schemas::event::*;
pub use schemas::puppet::PuppetOptions;
pub use types::{AsyncFnPtr, IntoAsyncFnPtr};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
