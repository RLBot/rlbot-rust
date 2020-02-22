//! ATBA, short for Always Towards Ball Agent, is a bot that always drives
//! blindly towards the ball no matter what is happening on the field (just
//! like Dory from Finding Nemo).

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::Vector2;
use rlbot::{flat, GameTickPacket};
use std::{error::Error, f32::consts::PI};

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    rlbot.start_match(&rlbot::MatchSettings::rlbot_vs_allstar("ATBA", "All-Star"))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next()?;

        // check that match is started and not showing a replay.
        // `packets.next_flatbuffer()` sleeps until the next packet is
        // available, so this loop will not roast your CPU :)
        if packet.game_info.is_round_active {
            let input = get_input(&packet);
            rlbot
                .interface()
                .update_player_input_flatbuffer(input.finished_data())?;
        }
    }
}

fn get_input<'a>(packet: &GameTickPacket) -> flatbuffers::FlatBufferBuilder<'a> {
    let ball = packet.ball.as_ref().expect("Missing ball");
    let ball_phys = &ball.physics;
    let ball_loc_3d = &ball_phys.location;
    let ball_loc = Vector2::new(ball_loc_3d.x, ball_loc_3d.y);

    let car = packet.players.get(0).expect("Missing player info");
    let car_phys = &car.physics;
    let car_loc_3d = &car_phys.location;
    let car_loc = Vector2::new(car_loc_3d.x, car_loc_3d.y);

    let offset = ball_loc - car_loc;
    let desired_yaw = f32::atan2(offset.y, offset.x);
    let car_rot = &car_phys.rotation;
    let steer = desired_yaw - car_rot.yaw;

    let player_index = 0;
    let controller_state_args = flat::ControllerStateArgs {
        throttle: 1.0,
        steer: normalize_angle(steer).max(-1.0).min(1.0),
        ..Default::default()
    };

    build_player_input(player_index, &controller_state_args)
}

fn build_player_input<'a>(
    player_index: i32,
    controller_state_args: &flat::ControllerStateArgs,
) -> flatbuffers::FlatBufferBuilder<'a> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let controller_state = Some(flat::ControllerState::create(
        &mut builder,
        &controller_state_args,
    ));

    let player_input = flat::PlayerInput::create(&mut builder, &flat::PlayerInputArgs {
        playerIndex: player_index,
        controllerState: controller_state,
    });

    builder.finish(player_input, None);
    builder
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
