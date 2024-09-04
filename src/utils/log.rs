use std::fmt::Display;
use tracing::{debug, error, info, trace, warn, Level};

/// Log
pub trait Log {
    fn log(self, level: Option<Level>);
}

impl<T, E: Display> Log for Result<T, E> {
    fn log(self, level: Option<Level>) {
        if let Err(error) = self {
            match level.unwrap_or(Level::ERROR) {
                Level::TRACE => trace!(%error),
                Level::DEBUG => debug!(%error),
                Level::INFO => info!(%error),
                Level::WARN => warn!(%error),
                Level::ERROR => error!(%error),
            }
        }
    }
}
