pub mod data;
pub mod service;
pub mod query;

pub use data::*;
pub use service::*;
pub use query::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
