pub(crate) mod generated;

pub mod api {
    pub use crate::generated::google_api::*;
}

pub mod rpc {
    pub use crate::generated::google_rpc::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
