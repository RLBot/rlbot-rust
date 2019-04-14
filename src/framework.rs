//! This module contains code for interoperating with RLBot's BotManager.

use crate::{ffi, init_with_options, InitOptions};
use std::{env, error::Error, path::PathBuf};

/// A bot that can run within the RLBot framework. Instances of `Bot` are used
/// by the [`run_bot`] function.
///
/// # Example
///
/// See [`examples/bot`] for a complete example.
///
/// [`examples/bot`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
pub trait Bot {
    /// This method is called when the bot's player index changes. The player
    /// index is the index in the
    /// [`LiveDataPacket::GameCars`](ffi::LiveDataPacket::GameCars) array which
    /// is under this `Bot`'s control. This method is guaranteed to be called
    /// before the first call to [`tick`](Bot::tick).
    fn set_player_index(&mut self, index: usize);

    /// This is called whenever there is a new game state. Your car will be
    /// controlled according to the [`PlayerInput`](ffi::PlayerInput) you
    /// return.
    fn tick(&mut self, packet: &ffi::LiveDataPacket) -> ffi::PlayerInput;
}

/// Runs a bot under control of the RLBot framework.
///
/// This function assumes the app was launched by the framework. It will
/// establish a connection to the framework, enter a game loop, and never
/// return.
///
/// # Errors
///
/// This function returns an error if it cannot communicate with the framework.
///
/// # Example
///
/// ```no_run
/// # use rlbot::ffi;
/// #
/// struct MyBot;
///
/// impl rlbot::Bot for MyBot {
///     // ...
///     # fn set_player_index(&mut self, index: usize) { unimplemented!() }
///     # fn tick(&mut self, packet: &ffi::LiveDataPacket) -> ffi::PlayerInput { unimplemented!() }
/// }
///
/// rlbot::run_bot(MyBot);
/// ```
///
/// See [`examples/bot`] for a complete example.
///
/// [`examples/bot`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
#[allow(deprecated)]
pub fn run_bot<B: Bot>(mut bot: B) -> Result<(), Box<dyn Error>> {
    let args = parse_framework_args()
        .map_err(|_| Box::<dyn Error>::from("could not parse framework arguments"))?
        .ok_or_else(|| Box::<dyn Error>::from("not launched by framework"))?;

    let player_index = args.player_index;

    let rlbot = init_with_options(args.into())?;

    bot.set_player_index(player_index as usize);

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next_ffi()?;
        let input = bot.tick(&packet);
        rlbot.interface().update_player_input(input, player_index)?;
    }
}

/// Parse the arguments passed by the RLBot framework.
///
/// This function returns:
///
/// * `Ok(Some(args))` – if the app was launched by the framework.
/// * `Ok(None)` – if the app was *not* launched by the framework.
/// * `Err(_)` – if it appears the app was launched by the framework, but we
///   could not understand the arguments.
pub fn parse_framework_args() -> Result<Option<FrameworkArgs>, ()> {
    parse_framework_command_line(env::args().skip(1))
}

fn parse_framework_command_line(
    mut args: impl Iterator<Item = String>,
) -> Result<Option<FrameworkArgs>, ()> {
    // Currently this only needs to interoperate with one caller – RLBot Python's
    // BaseSubprocessAgent. No public interface has been committed to, so we can
    // afford to be rigid and inflexible with the parsing.

    if args.next().as_ref().map(|s| &s[..]) != Some("--rlbot-version") {
        return Ok(None); // not launched by the framework
    }
    let rlbot_version = args.next().ok_or(())?;

    if args.next().as_ref().map(|s| &s[..]) != Some("--rlbot-dll-directory") {
        return Err(());
    }
    let rlbot_dll_directory = PathBuf::from(args.next().ok_or(())?);

    if args.next().as_ref().map(|s| &s[..]) != Some("--player-index") {
        return Err(());
    }
    let player_index = args.next().ok_or(())?.parse().map_err(|_| ())?;

    Ok(Some(FrameworkArgs {
        rlbot_version,
        rlbot_dll_directory,
        player_index,
        _non_exhaustive: (),
    }))
}

/// The arguments passed by the RLBot framework.
pub struct FrameworkArgs {
    /// The version of the RLBot framework used to launch the app. This is the
    /// same as the version shown when you run this Python code:
    ///
    /// ```python
    /// import rlbot
    /// print(rlbot.__version__)
    /// ```
    pub rlbot_version: String,

    /// The directory containing `RLBot_Core_Interface.dll` and
    /// `RLBot_Injector.exe`.
    pub rlbot_dll_directory: PathBuf,

    /// The index of the player you're controlling in the
    /// [`LiveDataPacket::GameCars`](ffi::LiveDataPacket::GameCars) array.
    pub player_index: i32,

    _non_exhaustive: (),
}

impl From<FrameworkArgs> for InitOptions {
    fn from(args: FrameworkArgs) -> Self {
        Self::new().rlbot_dll_directory(args.rlbot_dll_directory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pfcl(ss: Vec<&str>) -> Result<Option<FrameworkArgs>, ()> {
        parse_framework_command_line(ss.into_iter().map(str::to_string))
    }

    #[test]
    fn parse_framework_args() {
        let args = pfcl(vec![
            "--rlbot-version",
            "1.8.1",
            "--rlbot-dll-directory",
            "/tmp",
            "--player-index",
            "0",
        ])
        .unwrap()
        .unwrap();
        assert_eq!(args.rlbot_version, "1.8.1");
        assert_eq!(args.rlbot_dll_directory.to_str().unwrap(), "/tmp");
        assert_eq!(args.player_index, 0);
    }

    #[test]
    fn parse_empty_command_line() {
        let args = pfcl(vec![]).unwrap();
        assert!(args.is_none());
    }

    #[test]
    fn parse_non_matching_command_line() {
        let args = pfcl(vec!["--unrelated-argument"]).unwrap();
        assert!(args.is_none());
    }

    #[test]
    fn parse_error() {
        let args = pfcl(vec!["--rlbot-version"]);
        assert!(args.is_err());

        let args = pfcl(vec!["--rlbot-version", "1.8.1"]);
        assert!(args.is_err());
    }
}
