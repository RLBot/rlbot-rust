//! Draws a clock in the corner of the screen.

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::{error::Error, f32::consts::PI};

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

        let center_x = 100.0;
        let center_y = 150.0;

        let clock_hand = |fraction: f32, radius: f32| {
            let t = fraction * 2.0 * PI - PI / 2.0;
            (center_x + t.cos() * radius, center_y + t.sin() * radius)
        };

        let mut group = rlbot.begin_render_group(0);
        let green = group.color_rgb(0, 255, 0);
        group.draw_string_2d(
            (40.0, 20.0),
            (2, 2),
            format!("{}:{:02}.{:03}", min, sec, ms),
            green,
        );
        group.draw_line_2d(
            (center_x, center_y),
            clock_hand(min as f32 / 60.0, 60.0),
            green,
        );
        group.draw_line_2d(
            (center_x, center_y),
            clock_hand(sec as f32 / 60.0, 80.0),
            green,
        );
        group.draw_line_2d(
            (center_x, center_y),
            clock_hand(ms as f32 / 1000.0, 40.0),
            green,
        );
        group.render()?;
    }
}
