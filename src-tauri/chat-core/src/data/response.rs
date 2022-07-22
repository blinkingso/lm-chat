//! Types used by [`crate::Transport`]

use std::collections::HashMap;

use super::{channel, history, message::Message, object::Object, presence, timetoken::TimeToken};

/// A response to a publish request.
pub type Publish = TimeToken;

/// A response to a subscribe request.
pub type Subscribe = (Vec<Message>, TimeToken);

/// A response to a set state request.
pub type SetState = ();

/// A response to a get state request.
pub type GetState = Object;

/// A response to a hre now request.
pub type HereNow<T> = <T as presence::respond_with::RespondWith>::Response;

/// A response to a global here now request.
pub type GlobalHereNow<T> = presence::GlobalInfo<T>;

/// A response to a where now request, List of channels.
pub type WhereNow = Vec<channel::Name>;

/// A response to a heartbeat request.
pub type Heartbeat = ();

/// A response to a PAMv3 grant request.
pub type Grant = String;

/// A response to a get history request.
pub type GetHistory = HashMap<channel::Name, Vec<history::Item>>;

/// A response to a delete history request.
pub type DeleteHistory = ();

/// A response to a message counts request.
pub type MessageCounts = HashMap<channel::Name, usize>;

/// A response to a message counts with timetoken request.
pub type MessageCountsWithTimetoken = HashMap<channel::Name, usize>;

/// A response to a message counts with channel time-tokens request.
pub type MessageCountsWithChannelTimeTokens = HashMap<channel::Name, usize>;