//! Draws a clock in the corner of the screen.

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::{error::Error, f32::consts::PI};

const RADIUS: f32 = 250.0;

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    rlbot.start_match(&rlbot::MatchSettings::allstar_vs_allstar(
        "Leonardo",
        "Michelangelo",
    ))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next_flatbuffer()?;
        let mut total_ms = (packet.gameInfo().unwrap().secondsElapsed() * 1000.0) as i32;
        let ms = total_ms % 1000;
        total_ms -= ms;
        let sec = total_ms / 1000 % 60;
        total_ms -= sec * 1000;
        let min = total_ms / 1000 / 60;

        let center_x = 0.0;
        let center_y = -5120.0;
        let center_z = 1540.0;

        let clock_hand = |fraction: f32, radius: f32| {
            let t = fraction * 2.0 * PI - PI / 2.0;
            (
                center_x + t.cos() * radius,
                center_y,
                center_z - t.sin() * radius,
            )
        };

        let mut group = rlbot.begin_render_group(0);
        let green = group.color_rgb(0, 255, 0);
        let text = format!("{}:{:02}.{:03}", min, sec, ms);
        group.draw_string_3d(
            (
                center_x - 120.0 * text.len() as f32 / 2.0,
                center_y,
                center_z - RADIUS - 100.0,
            ),
            (2, 2),
            text,
            green,
        );
        group.draw_line_3d(
            (center_x, center_y, center_z),
            clock_hand(min as f32 / 60.0, RADIUS * 0.75),
            green,
        );
        group.draw_line_3d(
            (center_x, center_y, center_z),
            clock_hand(sec as f32 / 60.0, RADIUS),
            green,
        );
        group.draw_line_3d(
            (center_x, center_y, center_z),
            clock_hand(ms as f32 / 1000.0, RADIUS / 2.0),
            green,
        );
        group.render()?;
    }
}
