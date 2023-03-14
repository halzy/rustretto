use crate::MessageReceiver;

use bastion::prelude::*;
use futures_util::{future::Either, FutureExt};
use message::ClientId;

/// MessageListener is used to track view components mounting and unmounting.
///
/// When a component is mounted, the MessageListener will notify TBD so that
/// messages from the game can be routed to the view component.
#[derive(Clone)]
pub struct MessageListener {
    /// An identifier that all components in the same View have
    client_id: ClientId,
    supervisor: SupervisorRef,
}

impl MessageListener {
    pub fn new(client_id: ClientId, supervisor: SupervisorRef) -> Self {
        Self {
            client_id,
            supervisor,
        }
    }

    pub fn id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn listen<R>(&mut self, message_receiver: R) -> Result<ChildrenRef, ()>
    where
        R: MessageReceiver + Send + Sync + 'static + Clone,
    {
        tracing::error!("MessageListener mounted");

        let client_distributor = self.client_id.distributor();
        // Create a child that represents the UI component of this view
        self.supervisor.children(move |children| {
            children
                // When the game server has a message, it is sent to all the components with the same client_id
                // the components will then use only the messages that they need.
                .with_distributor(client_distributor)
                .with_exec(move |ctx| {
                    let message_receiver = message_receiver.clone();
                    async move {
                        // Wait for a BreachMessage that we can translate and send to the ViewHandle
                        loop {
                            let message_receiver = message_receiver.clone();
                            MessageHandler::new(ctx.recv().await?)
                                .on_tell(move |msg: R::Message, _sender| {
                                    Either::Left(async move { message_receiver.receive(msg).await })
                                })
                                .on_fallback(move |_msg, _sender| {
                                    tracing::warn!("MessageListener received unhandled message.");
                                    Either::Right(async move { Ok(()) })
                                })
                                .await
                                .inspect_err(|_e| {
                                    tracing::trace!("MessageListener message loop shutting down");
                                })?
                        }
                        #[allow(unreachable_code)]
                        Ok(())
                    }
                })
        })
    }
}
