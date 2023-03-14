use bastion::prelude::*;

pub fn start() -> Result<SupervisorRef, ()> {
    // Set up the web Distributor
    // let game_engine_group = Distributor::named("game_engine_group");

    // Set up the socket supervisor
    Bastion::supervisor(|sp| {
        // If the supervisor dies, restart just it
        let sp = sp.with_strategy(SupervisionStrategy::OneForOne);
        sp.children(|children| run(children))
    })
}

fn run(children: Children) -> Children {
    children
        .with_name("Game Server")
        .with_resizer(OptimalSizeExploringResizer::default())
        .with_distributor(Distributor::named("game_server"))
        .with_exec(move |ctx| async move {
            //
            MessageHandler::new(ctx.recv().await?)
                .on_tell(|msg: message::Message, sender| {
                    tracing::error!("Game Server received message: {msg:?}");
                })
                .on_fallback(|msg, sender| {
                    //
                    tracing::error!("Game Server received unhandled message: {msg:?}");
                });

            Ok(())
        })
}

// Show welcome text
// login || register
// login -> login text
// login -> enter game
// register -> registration flow
// registration flow -> character creation
// character creation -> enter game

// create a new struct Connection (?)
// Connection will direct messages to the correct backend and work through state changes?
