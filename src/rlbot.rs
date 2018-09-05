use dll::RLBotCoreInterface;
use error::RLBotError;
use ffi;
use inject;
use packeteer::Packeteer;
use std::cell::Cell;
use std::error::Error;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::ptr::null_mut;

/// Injects the RLBot core DLL into Rocket League, and initializes the interface
/// DLL. This function might sleep for a bit while it waits for RLBot to fully
/// initialize.
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
/// # fn main() -> Result<(), Box<::std::error::Error>> {
/// let rlbot = rlbot::init()?;
/// rlbot.start_match(rlbot::MatchSettings::simple_1v1("Hero", "Villain"))?;
///
/// let mut packets = rlbot.packeteer();
///
/// // Wait for the match to start. `packets.next()` sleeps until the next
/// // packet is available, so this loop will not roast your CPU :)
/// while !packets.next()?.GameInfo.RoundActive {}
///
/// loop {
///     let packet = packets.next()?;
///     let input: rlbot::PlayerInput = Default::default();
///     rlbot.update_player_input(input, 0)?;
/// }
/// # }
/// ```
///
/// See [`examples/simple`] for a complete example.
///
/// [`examples/simple`]: https://gitlab.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs
pub fn init() -> Result<RLBot, Box<Error>> {
    inject::inject_dll()?;

    let interface = RLBotCoreInterface::load()?;
    interface.wait_for_initialized()?;

    Ok(RLBot::new(interface))
}

/// The main interface to RLBot. All the RLBot calls that are available can be
/// made through this struct.
pub struct RLBot {
    interface: RLBotCoreInterface,
    /// I strongly doubt the RLBot DLL is thread-safe, so let's enforce that
    /// restriction.
    ///
    /// This is the abstruse equivalent of `impl !Sync` in nightly Rust.
    not_sync: PhantomData<Cell<()>>,
}

impl RLBot {
    pub(crate) fn new(interface: RLBotCoreInterface) -> RLBot {
        RLBot {
            interface,
            not_sync: PhantomData,
        }
    }

    /// Returns a [`Packeteer`] object, for conveniently accessing game state
    /// as it occurs.
    pub fn packeteer(&self) -> Packeteer {
        Packeteer::new(self)
    }

    /// Sends player input to RLBot.
    pub fn update_player_input(
        &self,
        player_input: ffi::PlayerInput,
        player_index: c_int,
    ) -> Result<(), RLBotError> {
        let status = (self.interface.update_player_input)(player_input, player_index);
        core_result(status)
    }

    /// Grabs the current [`LiveDataPacket`](ffi::LiveDataPacket) from RLBot.
    /// Consider using [`packeteer`](RLBot::packeteer) instead for a more
    /// convenient interface.
    pub fn update_live_data_packet(
        &self,
        packet: &mut ffi::LiveDataPacket,
    ) -> Result<(), RLBotError> {
        let status = (self.interface.update_live_data_packet)(packet);
        core_result(status)
    }

    /// Tell RLBot to start a match.
    pub fn start_match(&self, match_settings: ffi::MatchSettings) -> Result<(), RLBotError> {
        let status = (self.interface.start_match)(match_settings, None, null_mut());
        core_result(status)
    }
}

fn core_result(status: ffi::RLBotCoreStatus) -> Result<(), RLBotError> {
    match status {
        ffi::RLBotCoreStatus::Success => Ok(()),
        _ => Err(RLBotError { status }),
    }
}
