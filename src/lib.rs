pub(crate) mod generated;
mod client;

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
