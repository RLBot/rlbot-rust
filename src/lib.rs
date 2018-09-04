//! [![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
//! [![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
//! [![pipeline status](https://gitlab.com/whatisaphone/rlbot-rust/badges/master/pipeline.svg)](https://gitlab.com/whatisaphone/rlbot-rust/commits/master)
//!
//! <img src="https://gitlab.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" style="float:left;margin:0 16px 0 0" />
//! <img src="https://github.com/RLBot/RLBot/raw/674a96b3330cd4de80eb50458dae97488723e187/images/RLBot.png" height="96" style="float:right;margin:0 0 0 16px" />
//!
//! [RLBot] is a framework for creating offline Rocket League bots. This crate
//! exposes Rust bindings to RLBot's [RLBot_Core_Interface.dll]. It presents a
//! simple, safe interface that should feel comfortable to Rust developers.
//!
//! [RLBot]: https://github.com/RLBot/RLBot
//! [RLBot_Core_Interface.dll]: https://github.com/RLBot/RLBot/tree/master/src/main/cpp/RLBotInterface
//!
//! Most types in this crate come directly from RLBot, so for anything not
//! documented here, you'll need to use RLBot's docs as the authoritative
//! reference.
//!
//! When using this crate, you'll always start out by calling [`init`]. It will
//! return an [`RLBot`] instance with which you can begin using the framework.
//!
//! ## Quick start
//!
//! ```no_run
//! # fn main() -> Result<(), Box<::std::error::Error>> {
//! let rlbot = rlbot::init()?;
//! rlbot.start_match(rlbot::MatchSettings::simple_1v1("Hero", "Villain"))?;
//!
//! let mut packets = rlbot.packeteer();
//!
//! // Wait for the match to start. `packets.next()` sleeps until the next
//! // packet is available, so this loop will not roast your CPU :)
//! while !packets.next()?.GameInfo.RoundActive {}
//!
//! loop {
//!     let packet = packets.next()?;
//!     let input: rlbot::PlayerInput = Default::default();
//!     rlbot.update_player_input(input, 0)?;
//! }
//! # }
//! ```

extern crate libloading;
extern crate ratelimit;

use dll::RLBotCoreInterface;
pub use ffi::*;
pub use packeteer::Packeteer;
pub use rlbot::RLBot;
use std::error::Error;

mod dll;
mod error;
mod ffi;
mod ffi_impls;
mod inject;
mod packeteer;
mod rlbot;

/// Injects the RLBot core DLL into Rocket League, and initializes the interface
/// DLL. This function might sleep for a bit while it waits for RLBot to fully
/// initialize.
///
/// # Panics
///
/// Only one RLBot instance may be created over the life of the application. If
/// you call this function more than once, it will panic. If you lose the RLBot
/// instance, well, you should keep better track of your things.
pub fn init() -> Result<RLBot, Box<Error>> {
    inject::inject_dll()?;

    let interface = RLBotCoreInterface::load()?;
    interface.wait_for_initialized()?;

    Ok(RLBot::new(interface))
}
