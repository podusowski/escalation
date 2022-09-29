use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Protocol {
    Ping,
    Pong
}

/// `bson` crate can't serialize `enum` directly as it doesn't appear as
/// a "document" to it. This wrapper fixes it.
#[derive(Deserialize, Serialize)]
pub struct Message {
    pub value: Protocol
}
