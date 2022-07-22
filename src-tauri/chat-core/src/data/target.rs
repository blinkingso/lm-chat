//! Types and related types.

use super::channel;

/// Standard target represents a channel names list and a channel group names
/// list that together are suitable for use in the API calls.
/// The value of this type is guaranteed to fulfill the standard invariants
/// required by the API calls - that at least one channel or channel group has
/// to be specified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standard {
    /// The channel names you are subscribing to.
    channels: Vec<channel::Name>,

    /// The channel group names you are subscribing to.
    channel_groups: Vec<channel::Name>,
}


impl Standard {
    pub fn new(channels: Vec<channel::Name>, channel_groups: Vec<channel::Name>) -> Result<Self, (Vec<channel::Name>, Vec<channel::Name) {
        if channels.is_empty() && channel_groups.is_empty() {
            return Err((channels, channel_groups));
        }

        Ok(Self {
            channels,
            channel_groups
        })
    }

    #[must_use]
    pub fn into_inner(self) -> (Vec<channel::Name>, Vec<channel::Name>) {
        (self.channels, self.channel_groups)
    }
}