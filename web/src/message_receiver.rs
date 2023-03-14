use std::future::Future;

pub trait MessageReceiver: Send + 'static {
    type Message: std::fmt::Debug + Send;

    type Future<'a>: Future<Output = Result<(), ()>> + Send + 'a
    where
        Self: 'a;

    fn receive(&self, msg: Self::Message) -> Self::Future<'_>;
}
