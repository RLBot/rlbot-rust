//! [![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
//! [![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
//! [![Build Status](https://travis-ci.org/whatisaphone/rlbot-rust.svg?branch=master)](https://travis-ci.org/whatisaphone/rlbot-rust)
//!
//! <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" style="float:left;margin:0 16px 0 0" />
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
//! 1. [`run_bot`] and [`Bot`] – This is the **high-level** interface. It plays
//!    a single match from start to finish. It expects the app to have been
//!    launched by the RLBot framework, and runs its own game loop under
//!    framework control.
//! 2. [`init`] and [`RLBot`] – This is the **low-level** interface. You can use
//!    this to directly access the innards of RLBot for scripting, integration
//!    tests, or any other custom use-case.
//!
//! # Examples
//!
//! This crate comes with a few examples to get you started.
//!
//! * [`examples/bot`] – Demonstrates use of the [`run_bot`] API.
//! * [`examples/simple`] – Demonstrates use of the [`init`] API with plain
//!   structs.
//! * [`examples/simple_flatbuffer`] – Demonstrates use of the [`init`] API with
//!   FlatBuffers.
//! * [`examples/rendering`] – Demonstrates use of the
//!   [`RLBot::begin_render_group`] API.
//! * [`examples/gravity`] – Demonstrates use of the [`RLBot::set_game_state`]
//!   API.
//!
//! [`examples/bot`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
//! [`examples/simple`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs
//! [`examples/simple_flatbuffer`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple_flatbuffer.rs
//! [`examples/rendering`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/rendering.rs
//! [`examples/gravity`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/gravity.rs

#![cfg_attr(feature = "strict", deny(warnings, missing_docs))]

extern crate flatbuffers;
extern crate libloading;
#[cfg(feature = "use-nalgebra")]
extern crate nalgebra as na;
extern crate ratelimit;

pub mod ffi;
pub use framework::{run_bot, Bot};
pub use packeteer::Packeteer;
pub use physicist::Physicist;
pub use render::{Color, RenderGroup};
pub use rlbot::{init, RLBot};
pub use rlbot_generated::rlbot::flat;

mod dll;
mod error;
mod ffi_impls;
mod framework;
mod inject;
mod packeteer;
mod physicist;
mod render;
mod rlbot;
#[allow(non_camel_case_types, non_snake_case, missing_docs)]
mod rlbot_generated;
pub mod state;
#[cfg(feature = "use-nalgebra")]
mod state_nalgebra;
