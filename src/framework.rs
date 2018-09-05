//! This module contains code for interoperating with RLBot's BotManager.

use ffi;
use rlbot;
use std::env;
use std::error::Error;

/// A bot that can run within the RLBot framework. Instances of `Bot` are used
/// by the [`run`] function.
///
/// # Example
///
/// See [`examples/bot`] for a complete example.
///
/// [`examples/bot`]: https://gitlab.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
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
/// The argument is a function which creates a [`Bot`].
///
/// This function assumes the app was launched by the framework. It will
/// establish a connection to the framework, create a bot with the function you
/// pass in, enter a game loop, and never return.
///
/// # Errors
///
/// This function returns an error if it cannot communicate with the framework.
///
/// # Example
///
/// ```no_run
/// struct MyBot;
///
/// impl rlbot::Bot for MyBot {
///     // ...
///     # fn set_player_index(&mut self, index: usize) { unimplemented!() }
///     # fn tick(&mut self, packet: &rlbot::LiveDataPacket) -> rlbot::PlayerInput { unimplemented!() }
/// }
///
/// rlbot::run(|| MyBot);
/// ```
///
/// See [`examples/bot`] for a complete example.
///
/// [`examples/bot`]: https://gitlab.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs
pub fn run<B: Bot>(factory: impl Fn() -> B + Sync) -> Result<(), Box<Error>> {
    // Currently this only needs to interoperate with one caller – RLBot Python's
    // BaseSubprocessAgent. No public interface has been committed to, so we can
    // afford to be rigid and inflexible with the parsing.
    let mut args = env::args().skip(1);
    if args.next().as_ref().map(|s| &s[..]) != Some("--player-index") {
        return Err(protocol_err());
    }
    let player_index: i32 = args.next().ok_or_else(protocol_err)?.parse()?;

    let rlbot = rlbot::init()?;

    let mut bot = factory();
    bot.set_player_index(player_index as usize);

    let mut packets = rlbot.packeteer();

    loop {
        let packet = packets.next()?;
        let input = bot.tick(&packet);
        rlbot.update_player_input(input, player_index)?;
    }
}

fn protocol_err() -> Box<Error> {
    From::from("Framework protocol violation")
}
