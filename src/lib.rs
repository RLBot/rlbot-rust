//! [![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
//! [![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
//! [![pipeline status](https://gitlab.com/whatisaphone/rlbot-rust/badges/master/pipeline.svg)](https://gitlab.com/whatisaphone/rlbot-rust/commits/master)
//!
//! <img src="https://gitlab.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" style="float:left;margin:0 16px 0 0" />
//! <img src="https://github.com/RLBot/RLBot/raw/674a96b3330cd4de80eb50458dae97488723e187/images/RLBot.png" height="96" style="float:right;margin:0 0 0 16px" />
//!
//! [RLBot] is a framework for creating offline Rocket League bots. This crate
//! lets you write bots using a simple, safe interface that should feel
//! comfortable to Rust developers.
//!
//! [RLBot]: https://github.com/RLBot/RLBot
//!
//! Most types in this crate are exported directly from RLBot, so for anything
//! not documented here, you'll need to use RLBot's docs as the authoritative
//! reference.
//!
//! There are two ways to use this crate:
//!
//! 1. [`run`] and [`Bot`] – This is the **high-level** interface. It plays a
//!    single match from start to finish. It expects the app to have been
//!    launched by the RLBot framework, and runs its own game loop under
//!    framework control.
//! 2. [`init`] – This is the **low-level** interface. You can use this to
//!    directly access the innards of RLBot for scripting, integration tests, or
//!    any other custom use-case.
//!
//! # Examples
//!
//! This crate comes with a few examples to get you started.
//!
//! * [`examples/bot`] – Demonstrates use of the [`run`] API.
//! * [`examples/simple`] – Demonstrates use of the [`init`] API.
//!
//! [`examples/bot`]: https://gitlab.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
//! [`examples/simple`]: https://gitlab.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs

extern crate libloading;
extern crate ratelimit;

pub use ffi::*;
pub use framework::{run, Bot};
pub use packeteer::Packeteer;
pub use rlbot::{init, RLBot};

mod dll;
mod error;
mod ffi;
mod ffi_impls;
mod framework;
mod inject;
mod packeteer;
mod rlbot;
