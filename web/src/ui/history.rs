use crate::MessageListener;
use axum_live_view::{html, live_view::Updated, LiveView};
use serde::{Deserialize, Serialize};

pub(crate) struct History {
    message_listener: Option<MessageListener>,
}

impl History {
    pub fn new(message_listener: Option<MessageListener>) -> Self {
        Self { message_listener }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Message {}

impl LiveView for History {
    type Message = HistoryMsg;

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
            let result = message_listener.listen(move |message: breach::Message| {
                let handle = handle.clone();
                async move {
                    handle
                        .send(Self::Message::Breach(message))
                        .await
                        .map_err(|_| ())
                }
            });
            if let Err(err) = result {
                tracing::error!(?err, "Error mounting history component.");
                panic!("Error mounting history component. {:?}", err);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum HistoryMsg {
    Breach(breach::Message),
}
