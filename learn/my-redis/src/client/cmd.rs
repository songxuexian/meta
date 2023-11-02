use std::{num::ParseIntError, time::Duration};

use bytes::Bytes;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Ping {
        msg: Option<String>,
    },
    Get {
        key: String,
    },
    Set {
        key: String,
        #[clap(value_parser = bytes_from_str)]
        value: Bytes,
        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
    Publish {
        channel: String,
        #[clap(value_parser = bytes_from_str)]
        message: Bytes,
    },
    Subscribe {
        channels: Vec<String>,
    },
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Result<Bytes, String> {
    Ok(Bytes::from(src.to_string()))
}
