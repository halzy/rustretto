#![feature(result_option_inspect)]
#![feature(async_fn_in_trait)]
#![feature(type_alias_impl_trait)]
// #![feature(return_position_impl_trait_in_trait)]

mod message_listener;
mod message_receiver;
mod ui;

use message_listener::*;
use message_receiver::*;

use bastion::{
    prelude::*,
    supervisor::{SupervisionStrategy, SupervisorRef},
    Bastion,
};

static WEB_CLIENTS_BOOTSTRAP: &str = "web-clients-bootstrap";
static WEB_CLIENTS_GROUP: &str = "web-clients-group";

pub struct WebConfig {
    pub listen_port: u16,
}

pub fn start(config: WebConfig) -> Result<SupervisorRef, ()> {
    // Set up the web Distributor
    let web_clients_group = Distributor::named(WEB_CLIENTS_GROUP);
    let web_clients_bootstrap = Distributor::named(WEB_CLIENTS_BOOTSTRAP);

    // Set up the socket supervisor
    Bastion::supervisor(|sp| {
        // If the supervisor dies, restart just it
        let sp = sp.with_strategy(SupervisionStrategy::OneForOne);
        sp.children(|children| start_ui(config.listen_port, children))
    })
}

fn start_ui(listen_port: u16, children: Children) -> Children {
    children
        .with_name("Axum Server")
        .with_exec(move |_ctx| async move {
            ui::start(listen_port).await?;
            Ok(())
        })
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
