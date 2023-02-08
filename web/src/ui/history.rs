use axum_live_view::{extract::EmbedLiveView, html, live_view::Updated, LiveView};
use serde::{Deserialize, Serialize};

pub(crate) struct History {
    is_live: bool,
}

impl History {
    pub fn new<L>(embed_live_view: &EmbedLiveView<L>) -> Self {
        let is_live = embed_live_view.connected();

        Self { is_live }
    }
}

impl Drop for History {
    fn drop(&mut self) {
        tracing::error!("Live view is GONE+!!!!");
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Message {}

impl LiveView for History {
    type Message = HistoryMsg;

    fn update(
        self,
        msg: Self::Message,
        data: Option<axum_live_view::event_data::EventData>,
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
        uri: hyper::Uri,
        request_headers: &hyper::HeaderMap,
        handle: axum_live_view::live_view::ViewHandle<Self::Message>,
    ) {
        tracing::error!(?request_headers, "Live view history mounted");
        // Send a message to something that this component exists
        // Do we send it to a single child that represents this user?
        // HP UI
        // prompt
        // Location
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum HistoryMsg {}
