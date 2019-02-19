#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_player_input() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(&common::one_player_match())?;
        rlbot.wait_for_match_start()?;

        let mut packeteer = rlbot.packeteer();
        let start = packeteer.next()?;

        let input = rlbot::ControllerState {
            throttle: 1.0,
            ..Default::default()
        };
        rlbot.update_player_input(0, &input)?;

        thread::sleep(Duration::from_secs(1));
        let end = packeteer.next()?;

        // The car is facing the Y direction. It should be moving forward.
        assert!(end.players[0].physics.location.y > start.players[0].physics.location.y + 500.0);
        assert!(end.players[0].physics.velocity.y > 500.0);
        Ok(())
    })
}
