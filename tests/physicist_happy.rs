#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::error::Error;

mod common;

#[test]
fn integration_physicist_happy() -> Result<(), Box<Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        rlbot.start_match(rlbot::ffi::MatchSettings::rlbot_vs_allstar(
            "Hero", "Villain",
        ))?;
        rlbot.wait_for_match_start()?;

        let mut physicist = rlbot.physicist();
        let first = physicist.next()?;
        let second = physicist.next()?;
        assert!(second.Ball.State.Frame > first.Ball.State.Frame);
        Ok(())
    })
}
