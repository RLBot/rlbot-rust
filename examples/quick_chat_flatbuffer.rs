#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use rand::prelude::*;
use std::{error::Error, thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    rlbot.start_match(rlbot::ffi::MatchSettings::allstar_vs_allstar(
        "Noisy", "Silent",
    ))?;
    rlbot.wait_for_match_start()?;

    loop {
        sleep(Duration::from_secs(1));
        let result = rlbot.send_quick_chat(build_chat().finished_data());
        if let Err(error) = result {
            println!("{:?}", error);
        }
    }
}

fn build_chat() -> flatbuffers::FlatBufferBuilder<'static> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(64);
    let root = rlbot::flat::QuickChat::create(&mut builder, &rlbot::flat::QuickChatArgs {
        quickChatSelection: select_chat(),
        playerIndex: 0,
        teamOnly: false,
    });
    builder.finish(root, None);
    builder
}

fn select_chat() -> rlbot::flat::QuickChatSelection {
    let max_chat = rlbot::flat::QuickChatSelection::Custom_Compliments_Pro as i8 + 1;
    let chat_index = rand::thread_rng().gen_range(0, max_chat);
    unsafe { std::mem::transmute(chat_index) }
}
