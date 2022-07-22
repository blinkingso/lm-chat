type SubscribeLoopExitTx = String;
pub struct Builder<TTransport = (), TRuntime = ()> {
    /// Transport to use for communication.
    transport: TTransport,

    /// Runtime to use managing resources
    runtime: TRuntime,

    /// Subscription related configuration params.
    /// If set, gets a signal when subscribe loop exists.
    subscribe_loop_exit_tx: Option<SubscribeLoopExitTx>,
}
