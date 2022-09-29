use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Protocol {
    Ping,
    Pong
}
