#![cfg(windows)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

mod common;

#[test]
#[should_panic]
fn integration_init_twice() {
    common::with_rocket_league(|| {
        drop(rlbot::init());
        drop(rlbot::init());
    })
}
