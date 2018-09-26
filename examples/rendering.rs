//! Draws a clock in the corner of the screen.

extern crate nalgebra as na;
extern crate rlbot;

use rlbot::{ffi, flat};
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<Error>> {
    let rlbot = rlbot::init()?;

    let mut match_settings = ffi::MatchSettings::default();
    let players = ["Leonardo", "Michelangelo", "Donatello", "Raphael"];
    match_settings.NumPlayers = players.len() as i32;
    for (i, a) in players.iter().enumerate() {
        match_settings.PlayerConfiguration[i].Bot = true;
        match_settings.PlayerConfiguration[i].RLBotControlled = true;
        match_settings.PlayerConfiguration[i].set_name(a);
        match_settings.PlayerConfiguration[i].Team = (i % 2) as u8;
    }
    rlbot.start_match(match_settings)?;

    let mut packets = rlbot.packeteer();

    // Wait for the match to start. `packets.next()` sleeps until the next
    // packet is available, so this loop will not roast your CPU :)
    while !packets.next()?.GameInfo.RoundActive {}

    loop {
        let packet = packets.next()?;
        let mut total_ms = (packet.GameInfo.TimeSeconds * 1000.0) as i32;
        let ms = total_ms % 1000;
        total_ms -= ms;
        let sec = total_ms / 1000 % 60;
        total_ms -= sec * 1000;
        let min = total_ms / 1000 / 60;

        let center_x = 100.0;
        let center_y = 150.0;
        let r_min = 60.0;
        let r_sec = 80.0;
        let r_ms = 40.0;

        let mut group = rlbot.begin_render_group(0);
        let green = flat::ColorArgs::argb(255, 0, 255, 0);
        group.draw_string_2d(
            40.0,
            20.0,
            2,
            2,
            format!("{}:{:02}.{:03}", min, sec, ms),
            &green,
        );
        group.draw_line_2d(
            center_x,
            center_y,
            center_x + (min as f32 * 2.0 * PI / 60.0 - PI / 2.0).cos() * r_min,
            center_y + (min as f32 * 2.0 * PI / 60.0 - PI / 2.0).sin() * r_min,
            &green,
        );
        group.draw_line_2d(
            center_x,
            center_y,
            center_x + (sec as f32 * 2.0 * PI / 60.0 - PI / 2.0).cos() * r_sec,
            center_y + (sec as f32 * 2.0 * PI / 60.0 - PI / 2.0).sin() * r_sec,
            &green,
        );
        group.draw_line_2d(
            center_x,
            center_y,
            center_x + (ms as f32 * 2.0 * PI / 1000.0 - PI / 2.0).cos() * r_ms,
            center_y + (ms as f32 * 2.0 * PI / 1000.0 - PI / 2.0).sin() * r_ms,
            &green,
        );
        group.render()?;
    }
}
