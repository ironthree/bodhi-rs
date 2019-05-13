pub mod data;
pub mod query;
pub mod service;

pub use data::*;
pub use query::*;
pub use service::*;

#[cfg(test)]
mod tests {
    // TODO: move things from main.rs here
    // TODO: query *everything* from bodhi and make sure deserialization works

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
