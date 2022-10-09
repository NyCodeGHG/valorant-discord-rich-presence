use std::fmt::Display;

/// An event which the client can subscribe to.
#[derive(Debug)]
pub enum Event {
    Presences,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Presences => "OnJsonApiEvent_chat_v4_presences",
        };
        f.write_str(value)
    }
}
