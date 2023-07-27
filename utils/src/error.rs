use snafu::prelude::*;
use std::io;

#[derive(Debug, Snafu)]
pub enum InformError {
    #[snafu(display("Unable to send {} content \"{}\": {}", target, content, source))]
    SendFailure {
        source: io::Error,
        target: String,
        content: String,
    },
}

pub type Result<T, E = InformError> = std::result::Result<T, E>;
