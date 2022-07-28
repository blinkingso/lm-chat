use futures_channel::{mpsc, oneshot};
use futures_util::SinkExt;
use log::debug;

use crate::{
    data::pubsub,
    pubnub::PubNub,
    runtime::Runtime,
    subscription::subscribe_loop::{ControlCommand, SubscribeLoopParams},
    transport::Transport,
};

use super::{
    registry::Registry,
    subscribe_loop::{ControlTx, ExitTx},
    Subscription,
};

/// SubscribeLoopSupervisor is responsible for the
/// lifecycle of the subscribe loop.
///
/// It owns the subscribe loop control handle and provides
/// a high-level interface to the control operations.
/// It will intelligently spawn and re-spawn the subscribe loop on a need basis.
///
/// Deliberately doesn't implement `Clone` to avoid issues with improper
/// duplication of control handles.
#[derive(Debug)]
pub(crate) struct SubscribeLoopSupervisor {
    /// Configuration params.
    params: SubscribeLoopSupervisorParams,

    /// Control handle to the subscribe loop.
    control_tx: Option<ControlTx>,
}

/// SubscribeLoopSupervisorParams configuration params.
#[derive(Debug)]
pub(crate) struct SubscribeLoopSupervisorParams {
    /// If set, gets a signal when subscribe loop exits.
    pub exit_tx: Option<ExitTx>,
}

impl SubscribeLoopSupervisor {
    pub fn new(params: SubscribeLoopSupervisorParams) -> Self {
        Self {
            params,
            control_tx: None,
        }
    }
}

impl SubscribeLoopSupervisor {
    pub async fn subscribe<TTransport, TRuntime>(
        &mut self,
        pubnub: &mut PubNub<TTransport, TRuntime>,
        to: pubsub::SubscribeTo,
    ) -> Subscription<TRuntime>
    where
        TTransport: Transport + 'static,
        TRuntime: Runtime + 'static,
    {
        let (id, control_tx, channel_rx) = loop {
            let (channel_tx, channel_rx) = mpsc::channel(10);

            let id_or_retry = if let Some(ref mut control_tx) = self.control_tx {
                // Send a command to add the channel to the running subscribe loop.

                debug!("Adding destination {:?} to the running loop", to);

                let (id_tx, id_rx) = oneshot::channel();

                let control_command_result = control_tx
                    .send(ControlCommand::Add(to.clone(), channel_tx, id_tx))
                    .await;

                if control_command_result.is_err() {
                    self.control_tx = None;

                    debug!("Restarting the subscription loop");

                    None
                } else {
                    let id = id_rx.await.unwrap();

                    Some((id, control_tx.clone()))
                }
            } else {
                let mut registry = Registry::new();

                let (id, _) = registry.register(to.clone(), channel_tx);

                let (control_tx, control_rx) = mpsc::channel(10);
                let (ready_tx, ready_rx) = oneshot::channel();

                debug!("Creating the subscribe loop.");
                let subscribe_loop_params = SubscribeLoopParams {
                    control_rx,
                    ready_tx: Some(ready_tx),
                    exit_tx: self.params.exit_tx.clone(),
                    transport: pubnub.transport.clone(),
                    to: registry,
                };

                pubnub.runtime.spawn(subscribe_loop(subscribe_loop_params));

                debug!("Waiting for subscription loop ready...");
                ready_rx.await.expect("Unable to receive ready message");

                // Keep the control tx for later
                self.control_tx = Some(control_tx.clone());

                // Return the values from the loop
                Some((id, control_tx))
            };

            match id_or_retry {
                Some((id, control_tx)) => break (id, control_tx, channel_rx),
                None => continue,
            }
        };

        Subscription {
            runtime: pubnub.runtime.clone(),
            destination: to,
            id,
            control_tx,
            channel_rx,
        }
    }
}
