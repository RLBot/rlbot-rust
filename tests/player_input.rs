#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_player_input() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(common::one_player_match())?;
        rlbot.wait_for_match_start()?;

        let mut packeteer = rlbot.packeteer();
        let start = packeteer.next()?;

        let input = rlbot::ffi::PlayerInput {
            Throttle: 1.0,
            ..Default::default()
        };
        rlbot.update_player_input(input, 0)?;

        thread::sleep(Duration::from_secs(1));
        let end = packeteer.next()?;

        // The car should be accelerating forward.
        assert!(end.GameCars[0].Physics.Location.Y > start.GameCars[0].Physics.Location.Y);
        Ok(())
    })
}
