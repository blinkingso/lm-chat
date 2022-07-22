//! Publish/subscribe API related types.

use super::channel;

/// A subscription destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscribeTo {
    Channel(channel::Name),
    ChannelWildcard(channel::WildcardSpec),
    ChannelGroup(channel::Name),
}

impl SubscribeTo {
    #[must_use]
    pub fn as_channel(&self) -> Option<&channel::Name> {
        match self {
            SubscribeTo::Channel(ref n) => Some(n),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_channel_wildcard(&self) -> Option<&channel::WildcardSpec> {
        match self {
            SubscribeTo::ChannelWildcard(ref w) => Some(w),
            _ => None,
        }
    }

    pub fn as_channel_group(&self) -> Option<&channel::Name> {
        match self {
            SubscribeTo::ChannelGroup(ref n) => Some(n),
            _ => None,
        }
    }
}
