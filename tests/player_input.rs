#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate rlbot;
extern crate winapi;
extern crate winproc;

use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_player_input() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;
        let mut packeteer = rlbot.packeteer();

        rlbot.start_match(common::one_player_match())?;

        // Wait for the match to start.
        loop {
            let packet = packeteer.next()?;
            if packet.GameInfo.RoundActive && !packet.GameInfo.MatchEnded {
                break;
            }
        }

        let start = packeteer.next()?;

        let input = rlbot::ffi::PlayerInput {
            Throttle: 1.0,
            ..Default::default()
        };
        rlbot.update_player_input(input, 0)?;

        thread::sleep(Duration::from_secs(1));
        let end = packeteer.next()?;

        assert!(end.GameCars[0].Physics.Location.Y > start.GameCars[0].Physics.Location.Y);
        Ok(())
    })
}
