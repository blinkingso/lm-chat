use std::pin::Pin;

use futures_channel::mpsc;
use futures_util::{Stream, SinkExt};
use log::debug;

use super::subscribe_loop::{ChannelRx, ControlTx, SubscriptionID, ControlCommand};
use crate::data::{message::Message, pubsub};
use crate::runtime::Runtime;

#[derive(Debug)]
pub struct Subscription<TRuntime: Runtime> {
    /// Runtime to use for managing resources
    pub(crate) runtime: TRuntime,

    /// Subscription destination
    pub(crate) destination: pubsub::SubscribeTo,

    /// Unique identifier for the listener
    pub(crate) id: SubscriptionID,

    /// For cleaning up resources at the subscribe loop when dropped
    pub(crate) control_tx: ControlTx,

    /// Stream that produces messages.
    pub(crate) channel_rx: ChannelRx,
}

impl<TRuntime: Runtime> Stream for Subscription<TRuntime> {
    type Item = Message;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.get_mut().channel_rx), cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        Stream::size_hint(&self.channel_rx)
    }
}

impl<TRuntime: Runtime> Subscription<TRuntime> {
    fn drop_command(&self) -> ControlCommand {
        ControlCommand::Drop(self.id, self.destination.clone())
    }
}

impl<TRuntime: Runtime> Drop for Subscription<TRuntime> {
    fn drop(&mut self) {
        debug!("Dropping Subscription: {:?}", self.destination);

        let command = self.drop_command();
        let mut control_tx = self.control_tx.clone();
        self.runtime.spawn(async move {
            let drop_send_result = control_tx.send(command).await;
            assert!(!is_drop_send_result_error(drop_send_result), "Unable to unsubscribe")
        })
    }
}

fn is_drop_send_result_error(result: Result<(), mpsc::SendError>) -> bool {
    match result {
        Ok(_) => false,
        Err(err @ mpsc::SendError{..} ) if err.is_disconnected() => {
            // Control handle is dead, so we can assume loop is dead too,
            // and that we have, therefore, kind of unsubscribed successfully.
            false
        }
        Err(_) => true
    }
}