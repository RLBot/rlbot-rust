extern crate rlbot;
extern crate winapi;
extern crate winproc;

mod common;

#[test]
#[should_panic]
fn integration_init_twice() {
    common::with_rocket_league(|| {
        drop(rlbot::init());
        drop(rlbot::init());
    })
}
