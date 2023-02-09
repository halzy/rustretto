use axum_live_view::{extract::EmbedLiveView, html, live_view::Updated, LiveView};
use serde::{Deserialize, Serialize};
use util::ViewRegistrationGuard;

pub(crate) struct History {
    lifecycle: Option<ViewRegistrationGuard>,
}

impl History {
    pub fn new<L>(embed_live_view: &EmbedLiveView<L>, request_id: &util::ViewId) -> Self {
        let is_live = embed_live_view.connected();

        let lifecycle =
            is_live.then(|| ViewRegistrationGuard::new(util::ViewKind::History, request_id));

        Self { lifecycle }
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
        if let Some(lifecycle) = &self.lifecycle {
            lifecycle.mount(handle);
        }
        // Send a message to something that this component exists
        // Do we send it to a single child that represents this user?
        // HP UI
        // prompt
        // Location
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum HistoryMsg {}
