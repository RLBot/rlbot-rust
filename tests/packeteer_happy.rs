#![cfg(windows)]

extern crate rlbot;
extern crate winapi;
extern crate winproc;

use std::error::Error;

mod common;

#[test]
fn integration_packeteer_happy() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(rlbot::ffi::MatchSettings::simple_1v1("Hero", "Villain"))?;

        let mut packeteer = rlbot.packeteer();
        let first = packeteer.next()?.GameInfo.TimeSeconds;
        let second = packeteer.next()?.GameInfo.TimeSeconds;
        assert!(second > first);

        let first = packeteer
            .next_flatbuffer()?
            .gameInfo()
            .unwrap()
            .secondsElapsed();
        let second = packeteer
            .next_flatbuffer()?
            .gameInfo()
            .unwrap()
            .secondsElapsed();
        assert!(second > first);
        Ok(())
    })
}
