//! ATBA, short for Always Towards Ball Agent, is a bot that always drives
//! blindly towards the ball no matter what is happening on the field (just
//! like Dory from Finding Nemo).

#![cfg_attr(feature = "strict", deny(warnings))]

use na::Vector2;
use rlbot::ffi;
use std::{error::Error, f32::consts::PI};

fn main() -> Result<(), Box<Error>> {
    let rlbot = rlbot::init()?;
    rlbot.start_match(ffi::MatchSettings::simple_1v1("ATBA", "All-Star"))?;

    let mut packets = rlbot.packeteer();

    // Wait for the match to start. `packets.next()` sleeps until the next
    // packet is available, so this loop will not roast your CPU :)
    while !packets.next()?.GameInfo.RoundActive {}

    loop {
        let packet = packets.next()?;
        let input = get_input(&packet);
        rlbot.update_player_input(input, 0)?;
    }
}

fn get_input(packet: &ffi::LiveDataPacket) -> ffi::PlayerInput {
    let ball = packet.GameBall;
    let ball_loc = Vector2::new(ball.Physics.Location.X, ball.Physics.Location.Y);
    let car = packet.GameCars[0];
    let car_loc = Vector2::new(car.Physics.Location.X, car.Physics.Location.Y);

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let steer = desired_yaw - car.Physics.Rotation.Yaw;

    ffi::PlayerInput {
        Throttle: 1.0,
        Steer: normalize_angle(steer).max(-1.0).min(1.0),
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
