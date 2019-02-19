#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_set_game_state() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(&common::one_player_match())?;
        rlbot.wait_for_match_start()?;

        let desired_state = teleport_to_sky();
        rlbot.set_game_state(&desired_state)?;

        // Sometimes setting the state takes a few frames, so wait a bit.
        thread::sleep(Duration::from_millis(100));

        let packet = rlbot.packeteer().next_flatbuffer()?;
        let player = packet.players().unwrap().get(0);
        let player_loc_z = player.physics().unwrap().location().unwrap().z();
        assert!(player_loc_z > 1000.0);
        Ok(())
    })
}

fn teleport_to_sky() -> rlbot::DesiredGameState {
    rlbot::DesiredGameState::new().car_state(
        0,
        rlbot::DesiredCarState::new().physics(
            rlbot::DesiredPhysics::new()
                .location(rlbot::Vector3Partial::new().x(0.0).y(0.0).z(1500.0)),
        ),
    )
}
