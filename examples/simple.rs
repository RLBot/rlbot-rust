//! ATBA, short for Always Towards Ball Agent, is a bot that always drives
//! blindly towards the ball no matter what is happening on the field (just
//! like Dory from Finding Nemo).

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::Vector2;
use std::{error::Error, f32::consts::PI};

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    let player_index = 0;
    rlbot.start_match(rlbot::MatchSettings::rlbot_vs_allstar("ATBA", "All-Star"))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next()?;

        // check that match is started and not showing a replay.
        // `packets.next_flatbuffer()` sleeps until the next packet is
        // available, so this loop will not roast your CPU :)
        if packet.ball.is_some() {
            let input = get_input(&packet);
            rlbot.update_player_input(player_index, input)?;
        }
    }
}

fn get_input(packet: &rlbot::GameTickPacket) -> rlbot::ControllerState {
    let ball_loc = &packet.ball.as_ref().unwrap().physics.location;
    let ball_loc = Vector2::new(ball_loc.x, ball_loc.y);

    let car = &packet.players[0];
    let car_loc = &car.physics.location;
    let car_loc = Vector2::new(car_loc.x, car_loc.y);

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let steer = desired_yaw - car.physics.rotation.yaw;

    rlbot::ControllerState {
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
