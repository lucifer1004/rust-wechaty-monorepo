#[macro_use]
extern crate num_derive;

pub mod puppet;
pub mod schemas;
pub mod events;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
