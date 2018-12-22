#![cfg(windows)]
#![cfg_attr(feature = "strict", deny(warnings))]

mod common;

#[test]
#[should_panic]
fn integration_init_twice() {
    common::with_rocket_league(|| {
        drop(rlbot::init());
        drop(rlbot::init());
    })
}
