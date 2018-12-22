#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::error::Error;

mod common;

#[test]
fn integration_physicist_happy() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(rlbot::ffi::MatchSettings::simple_1v1("Hero", "Villain"))?;

        let mut physicist = rlbot.physicist();
        let first = physicist.next()?;
        let second = physicist.next()?;
        assert!(second.Ball.State.Frame > first.Ball.State.Frame);
        Ok(())
    })
}
