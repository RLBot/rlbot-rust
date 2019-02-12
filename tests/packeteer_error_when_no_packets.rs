#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::error::Error;

mod common;

#[test]
fn integration_packeteer_error_when_no_packets() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        // We never started a match, so no gameplay packets should ever come. We should
        // get *maybe* one initial empty packet, but after that, only errors.
        let mut packeteer = rlbot.packeteer();
        assert!(packeteer.next_flatbuffer().is_err() || packeteer.next_flatbuffer().is_err());
        Ok(())
    })
}
