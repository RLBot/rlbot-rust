#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use std::error::Error;

mod common;

#[test]
fn integration_physicist_flat_happy() -> Result<(), Box<dyn Error>> {
    common::with_rocket_league(|| {
        let rlbot = rlbot::init()?;

        #[allow(deprecated)]
        {
            rlbot
                .interface
                .start_match(rlbot::ffi::MatchSettings::rlbot_vs_allstar(
                    "Hero", "Villain",
                ))?;
        }
        rlbot.wait_for_match_start()?;

        let mut physicist = rlbot.physicist();
        let first = physicist.next_flat()?;
        let second = physicist.next_flat()?;

        let first_frame = first.ball().unwrap().state().unwrap().frame();
        let second_frame = second.ball().unwrap().state().unwrap().frame();
        assert!(second_frame > first_frame);
        Ok(())
    })
}
