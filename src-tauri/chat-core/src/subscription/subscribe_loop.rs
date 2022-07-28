use super::message_destination::MessageDestination;
use super::registry::Registry as GenericRegistry;
use crate::data::message::Message;
use crate::data::timetoken::TimeToken;
use crate::data::{pubsub, request, response};
use crate::transport::Service;
use futures_channel::{mpsc, oneshot};
use futures_util::future::{select, Either, FutureExt};
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use log::{debug, error};
use std::fmt::Debug;

pub(crate) use super::channel::{Rx as ChannelRx, Tx as ChannelTx};
pub(crate) use super::registry::ID as SubscriptionID;
pub(crate) type Registry = GenericRegistry<pubsub::SubscribeTo, ChannelTx>;
pub(crate) type ReadyTx = oneshot::Sender<()>;
pub(crate) type ExitTx = mpsc::Sender<()>;
pub(crate) type ControlTx = mpsc::Sender<ControlCommand>;
pub(crate) type ControlRx = mpsc::Receiver<ControlCommand>;
pub(crate) type SubscriptionIdTx = oneshot::Sender<SubscriptionID>;

#[derive(Debug)]
pub(crate) enum ControlCommand {
    /// A stream for a channel or channel group is being dropped.
    ///
    /// Only Sent from `Server` to `SubscribeLoop`.
    Drop(SubscriptionID, pubsub::SubscribeTo),

    /// A stream for a channel or channel group is being created.
    ///
    /// Only sent from `Server` to `SubscribeLoop`.
    Add(pubsub::SubscribeTo, ChannelTx, SubscriptionIdTx),
}

#[derive(Debug)]
pub(crate) struct SubscribeLoopParams<TTransport> {
    pub control_rx: ControlRx,
    pub ready_tx: Option<ReadyTx>,
    pub exit_tx: Option<ExitTx>,

    pub transport: TTransport,

    pub to: Registry,
}

#[derive(Debug)]
struct StateData {
    pub to: Registry,
}

pub(crate) async fn subscribe_loop<TTransport>(params: SubscribeLoopParams<TTransport>)
where
    TTransport: Service<request::Subscribe, Response = response::Subscribe> + Clone,
    <TTransport as Service<request::Subscribe>>::Error: Debug + 'static,
{
    debug!("Starting subscribe loop");

    let SubscribeLoopParams {
        mut control_rx,
        mut ready_tx,
        mut exit_tx,
        transport,
        to,
    } = params;

    let mut state_data = StateData { to };
    let mut timetoken = TimeToken::default();

    loop {
        let to: Vec<pubsub::SubscribeTo> = state_data.to.keys().cloned().collect();

        let request = request::Subscribe {
            to,
            timetoken,
            heartbeat: None,
        };

        let response = transport.call(request);
        let response = response.fuse();
        futures_util::pin_mut!(response);

        let control_rx_recv = control_rx.next();
        futures_util::pin_mut!(control_rx_recv);

        let (messages, next_timetoken) = match select(control_rx_recv, response).await {
            Either::Left((msg, _)) => {
                let outcome = handle_control_command(&mut state_data, msg).await;
                if let ControlOutcome::Terminate = outcome {
                    break;
                }

                continue;
            }
            Either::Right((res, _)) => match res {
                Ok(v) => v,
                Err(err) => {
                    error!("Transport error while polling: {:?}", err);
                    continue;
                }
            },
        };

        if let Some(ready_tx) = ready_tx.take() {
            if let Err(err) = ready_tx.send(()) {
                error!("Error sending ready message: {:?}", err);
                break;
            }
        }

        timetoken = next_timetoken;

        debug!("message: {:?}", messages);
        debug!("timetoken: {:?}", timetoken);

        dispatch_messages(&mut state_data, messages).await;
    }

    debug!("Stopping subscribe loop");

    if let Some(ref mut exit_tx) = exit_tx {
        exit_tx.send(()).await.expect("Unable to send exit message");
    }
}

#[derive(Debug)]
enum ControlOutcome {
    Terminate,
    CanContinue,
}

async fn handle_control_command(
    state_data: &mut StateData,
    msg: Option<ControlCommand>,
) -> ControlOutcome {
    debug!("Got request: {:?}", msg);

    let request = match msg {
        Some(v) => v,
        None => return ControlOutcome::CanContinue,
    };

    let StateData { to } = state_data;
    match request {
        ControlCommand::Drop(id, destination) => {
            debug!(
                "UnRegistering the listener at subscribe loop: {:?} {:?}",
                destination, id
            );

            let (_, _effect) = to
                .unregister(&destination, id)
                .expect("Unable to unregister destination from a subscribe loop");

            // TODO: avoid terminating loop here to avoid special casing.
            if to.is_empty() {
                ControlOutcome::Terminate
            } else {
                ControlOutcome::CanContinue
            }
        }

        ControlCommand::Add(destination, channel_tx, id_tx) => {
            debug!("Registering listener at subscribe loop: {:?}", destination);

            // Register the destination listener with the registry
            let (id, _effect) = to.register(destination, channel_tx);

            id_tx.send(id).expect("Unable to send subscription id");
            ControlOutcome::CanContinue
        }
    }
}

/// Dispatch messages to interested listeners.
async fn dispatch_messages(state_data: &mut StateData, messages: Vec<Message>) {
    // Distribute messages to each listener
    for message in messages {
        let destination = MessageDestination::new(&message);
        for des in destination {
            let listeners = state_data.to.get_iter_mut(&des);
            let listeners = match listeners {
                None => {
                    debug!("No listeners for message");
                    continue;
                }
                Some(v) => v,
            };

            debug!(
                "Delivering to {:?} listeners for {:?}...",
                listeners.size_hint(),
                des
            );

            for channel_tx in listeners {
                if let Err(error) = channel_tx.send(message.clone()).await {
                    error!("Delivery error: {:?}", error);
                }
            }
        }
    }
}
