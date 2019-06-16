#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::Vector2;
use std::{error::Error, f32::consts::PI};

fn main() -> Result<(), Box<dyn Error>> {
    rlbot::run_bot(MyBot { player_index: 0 })
}

struct MyBot {
    player_index: usize,
}

impl rlbot::Bot for MyBot {
    fn set_player_index(&mut self, index: usize) {
        self.player_index = index;
    }

    fn tick(&mut self, packet: &rlbot::GameTickPacket) -> rlbot::ControllerState {
        get_input(self.player_index, packet).unwrap_or_default()
    }
}

fn get_input(
    player_index: usize,
    packet: &rlbot::GameTickPacket,
) -> Option<rlbot::ControllerState> {
    let ball = packet.ball.as_ref()?;
    let ball_loc = Vector2::new(ball.physics.location.x, ball.physics.location.y);
    let car = &packet.players[player_index];
    let car_loc = Vector2::new(car.physics.location.x, car.physics.location.y);

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let steer = desired_yaw - car.physics.rotation.yaw;

    Some(rlbot::ControllerState {
        throttle: 1.0,
        steer: normalize_angle(steer).max(-1.0).min(1.0),
        ..Default::default()
    })
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
