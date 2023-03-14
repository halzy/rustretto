#![feature(result_option_inspect)]

use bastion::prelude::*;

pub fn start() -> Result<SupervisorRef, ()> {
    // Set up the socket supervisor
    Bastion::supervisor(|sp| {
        // If the supervisor dies, restart just it
        let sp = sp.with_strategy(SupervisionStrategy::OneForOne);
        sp.children(|children| run(children))
    })
}

fn run(children: Children) -> Children {
    children
        .with_name("Welcome Service")
        .with_resizer(OptimalSizeExploringResizer::default())
        .with_distributor(message::distributors::welcome())
        .with_exec(move |ctx| async move {
            MessageHandler::new(ctx.recv().await?)
                .on_tell(|msg: message::Welcome, _sender| {
                    tracing::error!("Welcome Server received message: {msg:?}");
                    match msg {
                        message::Welcome::Hello(hello) => say_hello(hello),
                    }
                })
                .on_fallback(|msg, _sender| {
                    tracing::error!("Welcome Server received unhandled message: {msg:?}");
                    Ok(())
                })
        })
}

fn say_hello(client_id: message::ClientId) -> Result<(), ()> {
    client_id
        .send(message::Client::Welcome("Heyo!!".into()))
        .inspect_err(|err| {
            tracing::error!(?err, %client_id, "Unable to welcome client!");
        })
        .map_err(|_| ())
}
