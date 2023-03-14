use bastion::prelude::*;

mod private {
    use crate::*;

    pub trait ConnectionState {}

    #[derive(Clone, Debug)]
    pub struct Start {
        pub client_id: ClientId,
    }
    pub struct Login {}
    pub struct Registration {}
    pub struct Game {}

    impl ConnectionState for Start {}
    impl ConnectionState for Login {}
    impl ConnectionState for Registration {}
    impl ConnectionState for Game {}
}

use private::*;

use crate::*;

pub struct Router<S: ConnectionState = Start> {
    extra: S,
}

impl Router<Start> {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            extra: Start { client_id },
        }
    }

    pub fn connect(&self) -> Result<(), SendError> {
        crate::distributors::welcome().tell_one(Welcome::Hello(self.extra.client_id.clone()))
    }
}
