use bastion::prelude::*;

pub fn game_server() -> Distributor {
    Distributor::named("game_server")
}

pub fn welcome() -> Distributor {
    Distributor::named("welcome_service")
}
