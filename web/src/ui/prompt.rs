use axum_live_view::{extract::EmbedLiveView, html, js_command, live_view::Updated, LiveView};

use serde::{Deserialize, Serialize};

pub(crate) struct Prompt {
    is_live: bool,
}

impl Prompt {
    pub fn new<L>(embed_live_view: &EmbedLiveView<L>) -> Self {
        let is_live = embed_live_view.connected();

        Self { is_live }
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
        uri: hyper::Uri,
        request_headers: &hyper::HeaderMap,
        handle: axum_live_view::live_view::ViewHandle<Self::Message>,
    ) {
        tracing::error!(?request_headers, "Live view prompt mounted");
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
