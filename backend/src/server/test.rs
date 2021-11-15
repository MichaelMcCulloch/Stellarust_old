#![cfg(test)]
use std::sync::Mutex;

use actix_web::web::Data;
use mockall::{mock, predicate::*};

use crate::broadcasterr::Broadcasterr;
use crate::server::Server;

mock! {
    pub Broadcasterr {}     // Name of the mock struct, less the "Mock" prefix
    impl Clone for Broadcasterr {   // specification of the trait to mock
        fn clone(&self) -> Self;
    }
}

use anyhow::Result;

#[test]
fn create_default__succeeds() -> Result<()> {
    let k = Data::new(Mutex::new(MockBroadcasterr::new()));
    assert!(Server::create_default(k).is_ok());
    Ok(())
}
