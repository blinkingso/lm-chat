use super::channel;
use super::object::Object;
use super::uuid::UUID;
use std::collections::HashMap;

pub mod respond_with {
    //! Type system level flags to specialize the response types.
    /// Return occupancy only
    #[derive(Debug, Clone, Copy)]
    pub struct OccupancyOnly;
    /// Return occupancy and UUIDs of the users.
    #[derive(Debug, Clone, Copy)]
    pub struct OccupancyAndUUIDs;

    /// Return occupancy, UUIDs of the users and the related states
    #[derive(Debug, Clone, Copy)]
    pub struct Full;

    /// A trait that bounds type system level flag to an actual type
    /// representing the response structure.
    pub trait RespondWith {
        type Response;
    }

    use super::{ChannelInfo, ChannelInfoWithOccupants, ChannelOccupantFullDetails, UUID};
    impl RespondWith for OccupancyOnly {
        type Response = ChannelInfo;
    }
    impl RespondWith for OccupancyAndUUIDs {
        type Response = ChannelInfoWithOccupants<UUID>;
    }

    impl RespondWith for Full {
        type Response = ChannelInfoWithOccupants<ChannelOccupantFullDetails>;
    }
}

/// Represents the channel info returned from here now
/// and global here now family of calls where only the occupants amount 
/// is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelInfo {
    pub occupancy: u64,
}

/// Represents the channel info returned from here now and 
/// global here now family of calls where the occupants amount
/// and per-occupant details are returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelInfoWithOccupants<T> {
    pub occupancy: u64,
    pub occupants: Vec<T>,
}

/// Represents the channel occupant details when the full data is requested.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelOccupantFullDetails {
    pub uuid: UUID,
    pub state: Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalInfo<T: respond_with::RespondWith> {
    pub total_channels: u64,
    pub total_occupancy: u64,
    pub channels: HashMap<channel::Name, T::Response>,
}

/// The heartbeat type alias. Used for heartbeats
pub type HeartbeatValue = u32;
