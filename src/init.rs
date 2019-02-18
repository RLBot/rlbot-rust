use crate::{dll::RLBotCoreInterface, inject, interface::RLBotInterface, rlbot::RLBot};
use std::{error::Error, path::PathBuf, thread::sleep, time::Duration};

/// Initializes RLBot and returns a ready-to-use [`RLBot`] object.
///
/// This function works exactly as [`init_with_options`]. Take a look there for
/// more details.
pub fn init() -> Result<RLBot, Box<dyn Error>> {
    init_with_options(Default::default())
}

/// Initializes RLBot and returns a ready-to-use [`RLBot`] object.
///
/// This function will inject the RLBot core DLL into Rocket League, and then
/// load the interface DLL. It might sleep for some time while it waits for
/// RLBot to fully initialize.
///
/// # Panics
///
/// Only one RLBot instance may be created over the life of the application. If
/// you call this function more than once, it will panic. If you lose the RLBot
/// instance, well, you should keep better track of your things.
///
/// # Example
///
/// ```no_run
/// # use rlbot::ffi::MatchSettings;
/// # use rlbot::flat;
/// #
/// # fn main() -> Result<(), Box<::std::error::Error>> {
/// let rlbot = rlbot::init()?;
/// rlbot.interface.start_match(MatchSettings::rlbot_vs_allstar("Hero", "Villain"))?;
/// rlbot.wait_for_match_start()?;
///
/// let mut packets = rlbot.packeteer();
/// loop {
///     let packet = packets.next_flatbuffer()?;
///     let input_args: flat::PlayerInputArgs = Default::default();
///     let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
///     let player_input = flat::PlayerInput::create(&mut builder, &input_args);
///     builder.finish(player_input, None);
///     rlbot.interface.update_player_input_flatbuffer(builder.finished_data())?;
/// }
/// # }
/// ```
///
/// See [`examples/simple`] for a complete example.
///
/// [`examples/simple`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs
#[allow(clippy::needless_pass_by_value)]
pub fn init_with_options(options: InitOptions) -> Result<RLBot, Box<dyn Error>> {
    let rlbot_dll_directory = options.rlbot_dll_directory.as_ref().map(|p| p.as_path());

    inject::inject_dll(rlbot_dll_directory)?;

    let dll = RLBotCoreInterface::load(rlbot_dll_directory)?;
    wait_for_initialized(&dll)?;

    Ok(RLBot::new(RLBotInterface::new(dll)))
}

fn wait_for_initialized(dll: &RLBotCoreInterface) -> Result<(), Box<dyn Error>> {
    for _ in 0..100 {
        if (dll.is_initialized)() {
            return Ok(());
        }
        sleep(Duration::from_millis(10));
    }

    Err(From::from("RLBot did not become initialized"))
}

/// Options for customizing the way the framework is initialized.
#[derive(Default)]
pub struct InitOptions {
    rlbot_dll_directory: Option<PathBuf>,
}

impl InitOptions {
    /// Constructs a new `InitOptions`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the directory in which to search for the RLBot DLLs.
    ///
    /// This should be set to the directory containing [these
    /// files][rlbot-dlls]. If this not set, the system's standard DLL search
    /// order will be used.
    ///
    /// [rlbot-dlls]: https://github.com/RLBot/RLBot/tree/cf5ca2794e153eef583eec093c2d9ea6e7afccd9/src/main/python/rlbot/dll
    pub fn rlbot_dll_directory(mut self, rlbot_dll_directory: impl Into<PathBuf>) -> Self {
        self.rlbot_dll_directory = Some(rlbot_dll_directory.into());
        self
    }
}
