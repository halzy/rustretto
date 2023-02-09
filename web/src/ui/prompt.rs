use util::{ViewKind, ViewRegistrationGuard};

use axum_live_view::{extract::EmbedLiveView, html, js_command, live_view::Updated, LiveView};

use serde::{Deserialize, Serialize};

pub(crate) struct Prompt {
    lifecycle: Option<util::ViewRegistrationGuard>,
}

impl Prompt {
    pub fn new<L>(embed_live_view: &EmbedLiveView<L>, request_id: &util::ViewId) -> Self {
        let is_live = embed_live_view.connected();

        let lifecycle = is_live.then(|| ViewRegistrationGuard::new(ViewKind::Prompt, request_id));

        Self { lifecycle }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Message {}

impl LiveView for Prompt {
    type Message = FormMsg;

    fn update(
        self,
        msg: Self::Message,
        data: Option<axum_live_view::event_data::EventData>,
    ) -> axum_live_view::live_view::Updated<Self> {
        let mut js_commands = Vec::new();

        match msg {
            FormMsg::Submit => {
                let new_msg = data
                    .unwrap()
                    .as_form()
                    .unwrap()
                    .deserialize::<Message>()
                    .unwrap();

                tracing::error!(something = ?new_msg, "We have a new message");

                js_commands.push(js_command::clear_value(".prompt"));
            }
            FormMsg::UserInputChange => {
                tracing::error!(?data, "Something happened!")
            }
        }

        Updated::new(self).with_all(js_commands)
    }

    fn render(&self) -> axum_live_view::Html<Self::Message> {
        html! {
            <form axm-submit={ FormMsg::Submit }>
                <span class="prompt-gt">
                    "&gt;"
                </span>
                <input
                    class="prompt"
                    type="text"
                    name="prompt"
                    placeholder="..."
                    axm-input={ FormMsg::UserInputChange}
                />
            </form>
        }
    }

    fn mount(
        &mut self,
        _uri: hyper::Uri,
        _request_headers: &hyper::HeaderMap,
        handle: axum_live_view::live_view::ViewHandle<Self::Message>,
    ) {
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
pub(crate) enum FormMsg {
    Submit,
    UserInputChange,
}
