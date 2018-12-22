#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::error::Error;

mod common;

#[test]
fn integration_physicist_flat_happy() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(rlbot::ffi::MatchSettings::simple_1v1("Hero", "Villain"))?;

        let mut physicist = rlbot.physicist();
        let first = physicist.next_flat()?;
        let second = physicist.next_flat()?;

        let first_frame = first.ball().unwrap().state().unwrap().frame();
        let second_frame = second.ball().unwrap().state().unwrap().frame();
        assert!(second_frame > first_frame);
        Ok(())
    })
}
