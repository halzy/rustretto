pub mod distributors;
pub mod router;

pub use router::Router;

use bastion::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientId(String);

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for ClientId {
    fn from(id: &str) -> Self {
        let id = id.to_owned();
        ClientId(id)
    }
}

impl ClientId {
    pub fn send(&self, m: Client) -> Result<(), SendError> {
        self.distributor().tell_one(m)
    }

    pub fn distributor(&self) -> Distributor {
        Distributor::named(&self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    Register(ClientId),
    RegistrationFailed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Welcome {
    Hello(ClientId),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Client {
    Welcome(String),
}
