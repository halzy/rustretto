use crate::MessageListener;

use axum_live_view::{
    html, js_command,
    live_view::{Updated, ViewHandle},
    LiveView,
};

use serde::{Deserialize, Serialize};

pub(crate) struct Prompt {
    message_listener: Option<MessageListener>,
}

impl Prompt {
    pub fn new(message_listener: Option<MessageListener>) -> Self {
        Self { message_listener }
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
                tracing::error!(?data, "Something happened!");
            }
            FormMsg::Breach(breach_message) => {
                tracing::error!(?breach_message, "breach message!");
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
        handle: ViewHandle<Self::Message>,
    ) {
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
                tracing::error!(?err, "Error mounting prompt component.");
                panic!("Error mounting prompt component. {:?}", err);
            }
        }
        // Send a message to something that this component exists
        // Do we send it to a single child that represents this user?
        // HP UI
        // prompt
        // Location
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FormMsg {
    Submit,
    UserInputChange,
    Breach(breach::Message),
}
