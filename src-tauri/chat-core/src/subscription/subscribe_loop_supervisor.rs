use super::subscribe_loop::{ControlTx, ExitTx};

/// SubscribeLoopSupervisor is responsible for the
/// lifecycle of the subscribe loop.
///
/// It owns the subscribe loop control handle and provides
/// a high-level interface to the control operations.
/// It will intelligently spawn and respawn the subscribe loop on a need basis.
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
