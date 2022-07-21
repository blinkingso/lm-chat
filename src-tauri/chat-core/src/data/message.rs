use super::channel;
use super::timetoken::TimeToken;
use json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Enum Type of Message
    pub message_type: Type,
    /// Wildcard channel of Message Receipt
    pub route: Option<Route>,
    /// Origin Channel of Message Receipt
    pub channel: channel::Name,
    /// Decoded JSON Message Payload
    pub json: JsonValue,
    /// Metadata of Message
    pub metadata: JsonValue,
    /// Message ID TimeToken
    pub timetoken: TimeToken,
    /// Issuing client ID.
    pub client: Option<String>,
    /// Subscribe key associated with the message.
    pub subscribe_key: String,
    /// Message flags
    pub flags: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Route {
    /// Message arrived on a wildcard channel.
    ChannelWildcard(channel::WildcardSpec),
    /// Message arrived on a channel group.
    ChannelGroup(channel::Name),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Type {
    /// A class message containing arbitrary payload data.
    Publish,
    /// A Lightweight message.
    Signal,
    /// An Objects service event, like space description updated.
    Objects,
    /// A message action event.
    Action,
    /// Presence event from channel(e.g. another client joined).
    Presence,
    // special meaning, now unknown.
    Unknown(u32),
}

impl Default for Message {
    #[must_use]
    fn default() -> Self {
        Self {
            message_type: Type::Unknown(0),
            route: None,
            channel: channel::Name::default(),
            json: JsonValue::Null,
            metadata: JsonValue::Null,
            timetoken: TimeToken::default(),
            client: None,
            subscribe_key: String::default(),
            flags: Default::default(),
        }
    }
}
