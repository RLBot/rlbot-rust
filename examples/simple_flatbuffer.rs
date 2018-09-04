//! ATBA, short for Always Towards Ball Agent, is a bot that always drives
//! blindly towards the ball no matter what is happening on the field (just
//! like Dory from Finding Nemo).

extern crate nalgebra;
extern crate rlbot;

use nalgebra::Vector2;
use rlbot::{
    ffi::MatchSettings,
    flat::{ControllerStateArgs, GameTickPacket},
};
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<Error>> {
    let rlbot = rlbot::init()?;
    rlbot.start_match(MatchSettings::simple_1v1("ATBA", "All-Star"))?;

    let mut packets = rlbot.packeteer();

    loop {
        let packet = packets.next_flatbuffer()?;

        // check that match is started and not showing a replay.
        // `packets.next_flatbuffer()` sleeps until the next packet is
        // available, so this loop will not roast your CPU :)
        if packet.gameInfo().unwrap().isRoundActive() {
            let input = get_input(&packet);
            let player_index = 0;
            rlbot.update_player_input_flatbuffer(player_index, input)?;
        }
    }
}

fn get_input(packet: &GameTickPacket) -> ControllerStateArgs {
    let ball = packet.ball().expect("Missing ball");
    let ball_phys = ball.physics().expect("Missing ball physics");
    let flat_ball_loc = ball_phys.location().expect("Missing ball location");
    let ball_loc = Vector2::new(flat_ball_loc.x(), flat_ball_loc.y());

    let car = packet.players().expect("Missing players").get(0);
    let car_phys = car.physics().expect("Missing player physics");
    let flat_car_loc = car_phys.location().expect("Missing player location");
    let car_loc = Vector2::new(flat_car_loc.x(), flat_car_loc.y());

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let flat_car_rot = car_phys.rotation().expect("Missing player rotation");
    let steer = desired_yaw - flat_car_rot.yaw();

    ControllerStateArgs {
        throttle: 1.0,
        steer: normalize_angle(steer).max(-1.0).min(1.0),
        ..Default::default()
    }
}

/// Normalize an angle to between -PI and PI.
fn normalize_angle(theta: f32) -> f32 {
    if theta < -PI {
        theta + (PI * 2.0)
    } else if theta >= PI {
        theta - (PI * 2.0)
    } else {
        theta
    }
}
