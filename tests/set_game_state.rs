#![cfg(windows)]
#![cfg(feature = "use-nalgebra")]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::Point3;
use rlbot::state;
use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_set_game_state() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(common::one_player_match())?;
        rlbot.wait_for_match_start()?;

        rlbot.set_game_state_struct(teleport_to_sky())?;

        // Sometimes setting the state takes a few frames, so wait a bit.
        thread::sleep(Duration::from_millis(100));

        let packet = rlbot.packeteer().next()?;
        assert!(packet.GameCars[0].Physics.Location.Z > 1000.0);
        Ok(())
    })
}

fn teleport_to_sky() -> state::DesiredGameState {
    let car_state = state::DesiredCarState::new()
        .physics(state::DesiredPhysics::new().location(Point3::new(0.0, 0.0, 1500.0)));
    state::DesiredGameState::new().car_state(0, car_state)
}
