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
//! ## Examples
//!
//! This crate comes with plenty examples to get you started. All the examples
//! can be run directly from the repo. For example, to run the `simple` example,
//! follow these steps:
//!
//! 1. Run Rocket League.
//!
//! 1. Run the example:
//!
//!    ```sh
//!    cargo run --example simple
//!    ```
//!
//! ### examples/simple ([Source][`examples/simple`])
//!
//! This is a simple ATBA, or **A**lways **T**owards **B**all **A**gent. It can
//! run with no dependencies other than RLBot itself.
//!
//! Key APIs:
//!
//! * [`init`]
//! * [`RLBot::start_match`]
//! * [`Packeteer::next`]
//! * [`RLBot::update_player_input`]
//!
//! ### examples/simple_flatbuffer ([Source][`examples/simple_flatbuffer`])
//!
//! Another ATBA, but using a secondary interface which uses flatbuffers. Many
//! of the low-level functions in RLBot's interface make use of flatbuffers.
//!
//! Key APIs:
//!
//! * [`Packeteer::next_flatbuffer`]
//! * [`RLBot::update_player_input_flatbuffer`]
//!
//! ### examples/rendering ([Source][`examples/rendering`])
//!
//! This example shows how to draw simple shapes to the game window. If you
//! don't see anything, try pressing PageUp, which is RLBot's shortcut for
//! turning on rendering.
//!
//! Key APIs:
//!
//! * [`RLBot::begin_render_group`]
//! * [`RenderGroup::render`]
//!
//! ### examples/gravity ([Source][`examples/gravity`])
//!
//! A fun example showing how to set game state.
//!
//! Key APIs:
//!
//! * [`RLBot::set_game_state_struct`]
//!
//! ### examples/gravity_flatbuffer ([Source][`examples/gravity_flatbuffer`])
//!
//! This works exactly the same as the previous example, except it uses the
//! low-level flatbuffer interface.
//!
//! Key APIs:
//!
//! * [`RLBot::set_game_state`]
//!
//! ### examples/bot ([Source][`examples/bot`])
//!
//! I saved the best for last. This is a full-fledged RLBot bot that can run
//! within the RLBot framework. It's different than the other examples, in that
//! it requires a working RLBot Python setup. Follow the instructions in
//! [RLBotPythonExample] to make sure you have all the necessary dependencies
//! installed. Once you have that working, you should be able to run a Rust bot
//! within the framework with this command:
//!
//! ```sh
//! cargo build --example bot && python -c "from rlbot import runner; runner.main()"
//! ```
//!
//! Key APIs:
//!
//! * [`run_bot`]
//!
//! [`examples/bot`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
//! [`examples/simple`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs
//! [`examples/simple_flatbuffer`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple_flatbuffer.rs
//! [`examples/rendering`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/rendering.rs
//! [`examples/gravity`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/gravity.rs
//! [`examples/gravity_flatbuffer`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/gravity_flatbuffer.rs
//! [RLBotPythonExample]: https://github.com/RLBot/RLBotPythonExample

#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(missing_docs, clippy::all)]
#![allow(intra_doc_link_resolution_failure)]

pub use crate::{
    framework::{parse_framework_args, run_bot, Bot, FrameworkArgs},
    init::{init, init_with_options, InitOptions},
    match_settings::*,
    packeteer::Packeteer,
    physicist::Physicist,
    render::{Color, RenderGroup},
    rlbot::RLBot,
    rlbot_generated::rlbot::flat,
};

mod dll;
mod error;
pub mod ffi;
mod ffi_impls;
mod framework;
mod init;
mod inject;
mod interface;
mod match_settings;
mod packeteer;
mod physicist;
mod render;
mod rlbot;
#[allow(non_camel_case_types, non_snake_case, missing_docs, clippy::all)]
mod rlbot_generated;
pub mod state;
mod state_convert;
#[cfg(feature = "use-nalgebra")]
mod state_nalgebra;
mod utils;
