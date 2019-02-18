//! The ball is a neutron star. The cars are planets.

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::{Unit, Vector3};
use rlbot::flat;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    rlbot.start_match(&rlbot::MatchSettings::allstar_vs_allstar("Earth", "Mars"))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    let mut i = 0;
    loop {
        let packet = packets.next_flatbuffer()?;

        // Check that the game is not showing a replay.
        // Also don't set state on every frame, that can make it laggy.
        if packet.gameInfo().unwrap().isRoundActive() && i % 8 == 0 {
            let desired_state = get_desired_state(&packet);
            rlbot
                .interface()
                .set_game_state(desired_state.finished_data())?;
        }
        i += 1;
    }
}

fn get_desired_state<'a>(packet: &flat::GameTickPacket<'_>) -> flatbuffers::FlatBufferBuilder<'a> {
    let ball = packet.ball().expect("Missing ball");
    let ball_phys = ball.physics().expect("Missing ball physics");
    let flat_ball_loc = ball_phys.location().expect("Missing ball location");
    let ball_loc = Vector3::new(flat_ball_loc.x(), flat_ball_loc.y(), flat_ball_loc.z());
    let cars = packet.players().expect("Missing players");

    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

    let mut car_offsets = Vec::with_capacity(cars.len());
    let mut i = 0;
    while i < cars.len() {
        let car = cars.get(i);
        let car_phys = car.physics().expect("Missing player physics");
        let v = car_phys.velocity().expect("Missing player velocity");
        let a = gravitate_towards_ball(&ball_loc, &car);

        let new_velocity = flat::Vector3Partial::create(&mut builder, &flat::Vector3PartialArgs {
            x: Some(&flat::Float::new(v.x() + a.x)),
            y: Some(&flat::Float::new(v.y() + a.y)),
            z: Some(&flat::Float::new(v.z() + a.z)),
        });

        let physics = flat::DesiredPhysics::create(&mut builder, &flat::DesiredPhysicsArgs {
            velocity: Some(new_velocity),
            ..Default::default()
        });

        let car_state = flat::DesiredCarState::create(&mut builder, &flat::DesiredCarStateArgs {
            physics: Some(physics),
            ..Default::default()
        });
        car_offsets.push(car_state);
        i += 1;
    }
    let car_states = builder.create_vector(&car_offsets);

    let desired_game_state =
        flat::DesiredGameState::create(&mut builder, &flat::DesiredGameStateArgs {
            carStates: Some(car_states),
            ..Default::default()
        });

    builder.finish(desired_game_state, None);
    builder
}

/// Generate an acceleration to apply to the car towards the ball, as if the
/// ball exerted a large gravitational force
fn gravitate_towards_ball(ball_loc: &Vector3<f32>, car: &flat::PlayerInfo<'_>) -> Vector3<f32> {
    let car_phys = car.physics().expect("Missing player physics");
    let flat_car_loc = car_phys.location().expect("Missing player location");
    let car_loc = Vector3::new(flat_car_loc.x(), flat_car_loc.y(), flat_car_loc.z());
    let ball_delta = ball_loc - car_loc;
    let distance = ball_delta.norm();
    let k = 1_000_000.0;
    let a = k / (distance / 5.0).powf(2.0);
    a * Unit::new_normalize(ball_delta).into_inner()
}
