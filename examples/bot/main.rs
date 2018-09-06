extern crate nalgebra;
extern crate rlbot;

use nalgebra::Vector2;
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<Error>> {
    rlbot::run(MyBot { player_index: 0 })
}

struct MyBot {
    player_index: usize,
}

impl rlbot::Bot for MyBot {
    fn set_player_index(&mut self, index: usize) {
        self.player_index = index;
    }

    fn tick(&mut self, packet: &rlbot::LiveDataPacket) -> rlbot::PlayerInput {
        get_input(self.player_index, packet)
    }
}

fn get_input(player_index: usize, packet: &rlbot::LiveDataPacket) -> rlbot::PlayerInput {
    let ball = packet.GameBall;
    let ball_loc = Vector2::new(ball.Physics.Location.X, ball.Physics.Location.Y);
    let car = packet.GameCars[player_index];
    let car_loc = Vector2::new(car.Physics.Location.X, car.Physics.Location.Y);

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let steer = desired_yaw - car.Physics.Rotation.Yaw;

    rlbot::PlayerInput {
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
