use crate::data::message::{self, Message};
use crate::data::pubsub;

#[derive(Debug)]
pub(super) struct MessageDestination<'a> {
    message: &'a Message,
    route: bool,
    channel: bool,
}

impl<'a> MessageDestination<'a> {
    pub fn new(message: &'a Message) -> Self {
        MessageDestination {
            message,
            route: false,
            channel: false,
        }
    }

    fn route_destination(&self) -> Option<pubsub::SubscribeTo> {
        match self.message.route {
            Some(message::Route::ChannelGroup(ref val)) => {
                Some(pubsub::SubscribeTo::ChannelGroup(val.clone()))
            }
            Some(message::Route::ChannelWildcard(ref val)) => {
                Some(pubsub::SubscribeTo::ChannelWildcard(val.clone()))
            }
            _ => None,
        }
    }
}

impl<'a> Iterator for MessageDestination<'a> {
    type Item = pubsub::SubscribeTo;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.route {
            self.route = true;
            let destination = self.route_destination();
            if destination.is_some() {
                return destination;
            }
        }
        if !self.channel {
            self.channel = true;
            return Some(pubsub::SubscribeTo::Channel(self.message.channel.clone()));
        }

        None
    }
}
