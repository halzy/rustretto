use breach::ViewId;

use bastion::prelude::*;
use futures_util::future::FutureExt;

use std::future::Future;

/// MessageListener is used to track view components mounting and unmounting.
///
/// When a component is mounted, the MessageListener will notify TBD so that
/// messages from the game can be routed to the view component.
#[derive(Clone)]
pub struct MessageListener {
    /// An identifier that all components in the same View have
    view_id: ViewId,
    supervisor: SupervisorRef,
}

impl MessageListener {
    pub fn new(view_id: ViewId, supervisor: SupervisorRef) -> Self {
        Self {
            view_id,
            supervisor,
        }
    }

    pub fn listen<F, G>(&mut self, message_handler: F) -> Result<ChildrenRef, ()>
    where
        F: Fn(breach::Message) -> G + Send + 'static + Clone,
        G: Future<Output = Result<(), ()>> + Send + 'static,
    {
        tracing::error!("MessageListener mounted");

        // When the game server has a message, it is sent to all the components with the same view_id
        // the components will then use only the messages that they need.
        let distributor = Distributor::named(self.view_id.as_ref());

        // Create a child that represents the UI component of this view
        self.supervisor.children(move |children| {
            children
                .with_distributor(distributor)
                .with_exec(move |ctx| {
                    let message_handler = message_handler.clone();
                    async move {
                        // Wait for a BreachMessage that we can translate and send to the ViewHandle
                        loop {
                            let message_handler = message_handler.clone();
                            MessageHandler::new(ctx.recv().await?)
                                .on_tell(|msg, _sender| {
                                    async move {
                                        let handler = message_handler(msg);
                                        handler.await.inspect_err(|_e| {
                                            tracing::trace!(
                                                "Prompt View message loop shutting down"
                                            );
                                        })
                                    }
                                    .boxed()
                                })
                                .on_fallback(|_msg, _sender| {
                                    tracing::warn!("MessageSubscriber received unhandled message.");
                                    async move { Ok(()) }.boxed()
                                })
                                .await?
                        }
                        #[allow(unreachable_code)]
                        Ok(())
                    }
                })
        })
    }
}
