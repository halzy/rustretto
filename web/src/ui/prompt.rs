use crate::{message_receiver::MessageReceiver, MessageListener};

use axum_live_view::{
    html, js_command,
    live_view::{Updated, ViewHandle},
    LiveView,
};

use message::Message;
use serde::{Deserialize, Serialize};

pub(crate) struct Prompt {
    message_listener: Option<MessageListener>,
}

impl Prompt {
    pub fn new(message_listener: Option<MessageListener>) -> Self {
        Self { message_listener }
    }
}

impl LiveView for Prompt {
    type Message = ViewMsg;

    fn update(
        self,
        msg: Self::Message,
        data: Option<axum_live_view::event_data::EventData>,
    ) -> axum_live_view::live_view::Updated<Self> {
        let mut js_commands = Vec::new();

        match msg {
            ViewMsg::Submit => {
                let new_msg = data
                    .unwrap()
                    .as_form()
                    .unwrap()
                    .deserialize::<FormData>()
                    .unwrap();

                tracing::error!(something = ?new_msg, "We have a new message");

                js_commands.push(js_command::clear_value(".prompt"));
            }
            ViewMsg::UserInputChange => {
                tracing::error!(?data, "Something happened!");
            }
            ViewMsg::Something => todo!(),
        }

        Updated::new(self).with_all(js_commands)
    }

    fn render(&self) -> axum_live_view::Html<Self::Message> {
        html! {
            <form axm-submit={ ViewMsg::Submit }>
                <span class="prompt-gt">
                    "&gt;"
                </span>
                <input
                    class="prompt"
                    type="text"
                    name="prompt"
                    placeholder="..."
                    axm-input={ ViewMsg::UserInputChange}
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
            let result = message_listener.listen(Receiver::new(handle));
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct FormData {
    #[allow(dead_code)]
    prompt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum ViewMsg {
    Submit,
    UserInputChange,
    Something,
}
