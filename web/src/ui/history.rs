use crate::{message_receiver::MessageReceiver, MessageListener};
use axum_live_view::{
    html,
    live_view::{Updated, ViewHandle},
    LiveView,
};
use message::Message;
use serde::{Deserialize, Serialize};

pub(crate) struct History {
    message_listener: Option<MessageListener>,
}

impl History {
    pub fn new(message_listener: Option<MessageListener>) -> Self {
        Self { message_listener }
    }
}

impl LiveView for History {
    type Message = ViewMsg;

    fn update(
        self,
        _msg: Self::Message,
        _data: Option<axum_live_view::event_data::EventData>,
    ) -> axum_live_view::live_view::Updated<Self> {
        Updated::new(self)
    }

    fn render(&self) -> axum_live_view::Html<Self::Message> {
        html! {
            <div>"history goes here!"</div>
        }
    }

    fn mount(
        &mut self,
        _uri: hyper::Uri,
        request_headers: &hyper::HeaderMap,
        handle: axum_live_view::live_view::ViewHandle<Self::Message>,
    ) {
        tracing::error!(?request_headers, "Live view history mounted");
        if let Some(message_listener) = &mut self.message_listener {
            let result = message_listener.listen(Receiver::new(handle));
            if let Err(err) = result {
                tracing::error!(?err, "Error mounting history component.");
                panic!("Error mounting history component. {:?}", err);
            }
        }
    }
}

#[derive(Clone)]
struct Receiver {
    handle: ViewHandle<ViewMsg>,
}

impl Receiver {
    fn new(handle: ViewHandle<ViewMsg>) -> Self {
        Self { handle }
    }
}

#[async_trait::async_trait]
impl MessageReceiver for Receiver {
    async fn receive(&self, _msg: Message) -> Result<(), ()> {
        // FIXME: use a real messag
        self.handle.send(ViewMsg::Something).await.map_err(|_| ())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum ViewMsg {
    Something,
}
