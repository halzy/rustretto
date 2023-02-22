use message::Message;

use async_trait::async_trait;

#[async_trait]
pub trait MessageReceiver {
    async fn receive(&self, msg: Message) -> Result<(), ()>;
}
