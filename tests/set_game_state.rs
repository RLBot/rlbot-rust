#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate flatbuffers;
extern crate rlbot;
extern crate winapi;
extern crate winproc;

use rlbot::state;
use std::error::Error;
use std::thread;
use std::time::Duration;

mod common;

#[test]
fn integration_set_game_state() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;
        rlbot.start_match(common::one_player_match())?;
        rlbot.wait_for_match_start()?;
        rlbot.set_game_state_struct(teleport_to_sky())?;
        thread::sleep(Duration::from_millis(100));

        let packet = rlbot.packeteer().next()?;
        assert!(packet.GameCars[0].Physics.Location.Z > 1000.0);
        Ok(())
    })
}

fn teleport_to_sky() -> state::DesiredGameState {
    let car_state = state::DesiredCarState::new().physics(
        state::DesiredPhysics::new().location(state::Vector3Partial::new().x(0.0).y(0.0).z(1500.0)),
    );
    state::DesiredGameState::new().car_state(0, car_state)
}
