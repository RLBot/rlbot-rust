#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::error::Error;

mod common;

#[test]
fn integration_packeteer_happy() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(rlbot::MatchSettings::rlbot_vs_allstar("Hero", "Villain"))?;
        rlbot.wait_for_match_start()?;

        let mut packeteer = rlbot.packeteer();
        let first = packeteer
            .next_flatbuffer()?
            .gameInfo()
            .unwrap()
            .secondsElapsed();
        let second = packeteer
            .next_flatbuffer()?
            .gameInfo()
            .unwrap()
            .secondsElapsed();
        assert!(second > first);
        Ok(())
    })
}
