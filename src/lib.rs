#[macro_use]
extern crate tracing;
pub(crate) mod cached;
mod client;
pub(crate) mod generated;

pub use client::*;

pub mod api {
    pub use crate::generated::google_api::*;
}

pub mod rpc {
    pub use crate::generated::google_rpc::*;
}

pub type Result<A> = std::result::Result<A, client::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
