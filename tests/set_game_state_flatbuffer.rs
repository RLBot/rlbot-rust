#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::{error::Error, thread, time::Duration};

mod common;

#[test]
fn integration_set_game_state() -> Result<(), Box<Error>> {
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

        let desired_state = teleport_to_sky();
        rlbot.set_game_state(desired_state.finished_data())?;

        thread::sleep(Duration::from_millis(100));

        let packet = packeteer.next()?;
        assert!(packet.GameCars[0].Physics.Location.Z > 1000.0);
        Ok(())
    })
}

fn teleport_to_sky() -> flatbuffers::FlatBufferBuilder<'static> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let location =
        rlbot::flat::Vector3Partial::create(&mut builder, &rlbot::flat::Vector3PartialArgs {
            x: Some(&rlbot::flat::Float::new(0.0)),
            y: Some(&rlbot::flat::Float::new(0.0)),
            z: Some(&rlbot::flat::Float::new(1500.0)),
        });
    let physics =
        rlbot::flat::DesiredPhysics::create(&mut builder, &rlbot::flat::DesiredPhysicsArgs {
            location: Some(location),
            ..Default::default()
        });
    let car_state =
        rlbot::flat::DesiredCarState::create(&mut builder, &rlbot::flat::DesiredCarStateArgs {
            physics: Some(physics),
            ..Default::default()
        });
    let car_states = builder.create_vector(&[car_state]);
    let desired_game_state =
        rlbot::flat::DesiredGameState::create(&mut builder, &rlbot::flat::DesiredGameStateArgs {
            carStates: Some(car_states),
            ..Default::default()
        });
    builder.finish(desired_game_state, None);
    builder
}
