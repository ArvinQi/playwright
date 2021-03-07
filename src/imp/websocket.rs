use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct WebSocket {
    channel: ChannelOwner
}

impl WebSocket {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for WebSocket {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}