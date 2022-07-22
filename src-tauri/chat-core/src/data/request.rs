//! Types used by [`crate::Transport`]

use std::collections::HashMap;
use std::marker::PhantomData;

use super::object::Object;
use super::timetoken::TimeToken;
use super::uuid::UUID;
use super::{channel, history, pam, presence, pubsub};

/// A request to publish a message to a channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Publish {
    /// A channel name to publish the message to.
    pub channel: channel::Name,

    /// The body of the message,
    pub payload: Object,

    /// Additional information associated the the message
    pub meta: Option<Object>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscribe {
    /// The destinations to subscribe to.
    pub to: Vec<pubsub::SubscribeTo>,

    /// A timetoken to use.
    /// tt: 0(zero) for the initial subscribe, or a valid timetoken if
    /// resuming / continuing / fast-forwarding from a previous subscribe flow.
    /// tr: Region as returned from the initial call with tt=0.
    pub timetoken: TimeToken,

    /// The heartbeat value to send to the network
    pub heartbeat: Option<presence::HeartbeatValue>,
}

/// Set state for a user for channels and/or channel groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetState {
    /// The channel names to set state for.
    pub channels: Vec<channel::Name>,

    /// The channel group names to set state for.
    pub channel_groups: Vec<channel::Name>,

    /// The user UUID to set state for.
    pub uuid: UUID,

    /// State to set.
    pub state: Object,
}

/// Get state for a user for channels and/or channel groups
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetState {
    pub channels: Vec<channel::Name>,

    pub channel_groups: Vec<channel::Name>,

    pub uuid: UUID,
}

/// Retrieve UUID and State Information for subscribed devices on a specific channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HereNow<TRespondWith>
where
    TRespondWith: presence::respond_with::RespondWith,
{
    /// The channel names to get the state for.
    pub channels: Vec<channel::Name>,

    /// The channel group name to get state for.
    pub channel_groups: Vec<channel::Name>,

    /// Type that specializes the response type.
    pub respond_with: PhantomData<TRespondWith>,
}

/// Retrieve UUID and State Information for subscribed devices on a all channels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalHereNow<TRespondWith>
where
    TRespondWith: presence::respond_with::RespondWith,
{
    /// Type that specializes the response type.
    pub respond_with: PhantomData<TRespondWith>,
}

/// Get list of channels user is present in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereNow {
    /// The User UUID to get list of channels for.
    pub uuid: UUID,
}

/// Announce a heartbeat
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    /// The presence timeout period. If `None`, the default value is used.
    pub heartbeat: Option<presence::HeartbeatValue>,

    /// The subscription destinations to announce heartbeat for.
    pub to: Vec<pubsub::SubscribeTo>,

    /// The User UUID to announce subscription for.
    pub uuid: UUID,

    /// State to set for channels and channel groups.
    pub state: Object,
}

pub type Grant = pam::GrantBody;

/// Fetch history
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetHistory {
    /// The channel names to get the history for.
    pub channels: Vec<channel::Name>,

    /// The batch history is limited to 500 channels and only the last 25
    /// messages per channel.
    pub max: Option<usize>,

    /// Direction of time traversal. Default is false, which means timeline is
    /// traversed newest to oldest
    pub reverse: Option<bool>,

    /// If provided, lets you select a "start date", in TimeToken format.
    /// If no provided, it will default to current time.
    /// Page through results by providing a start OR end time token.
    /// Retrieve a slice of the time line by providing both a start AND end time token.
    /// Start is "exclusive" - that is, the first item returned will be
    /// the one immediately after the start TimeToken value.
    pub start: Option<history::TimeToken>,

    /// If provide, lets you select a "end date", in TimeToken format.
    /// If not provided, it will default to current time.
    /// Page through results by providing a start OR end time token.
    /// Retrieve a slice of the time line by providing both a start AND end time token.
    /// End is "exclusive" - that is, if a message is associated exactly with
    /// the end TimeToken, it will be included in the result.
    pub end: Option<history::TimeToken>,

    /// Whether to request metadata to the populated in the returned items or not.
    pub include_metadata: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteHistory {
    pub channels: Vec<channel::Name>,
    pub start: Option<history::TimeToken>,
    pub end: Option<history::TimeToken>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageCountsWithTimeToken {
    pub channels: Vec<channel::Name>,
    pub timetoken: history::TimeToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageCountsWithChannelTimeTokens {
    /// A list of channels with time-tokens to get message counts at.
    /// TimeToken value must be non-zero.
    pub channels: HashMap<channel::Name, history::TimeToken>,
}
