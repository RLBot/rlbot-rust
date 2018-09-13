use dll::RLBotCoreInterface;
use error::RLBotError;
use ffi;
use flatbuffers;
use inject;
use packeteer::Packeteer;
use rlbot_generated::rlbot::flat;
use std::cell::Cell;
use std::error::Error;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::slice;

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
/// use rlbot::ffi::MatchSettings;
/// use rlbot::flat::{ControllerStateArgs, GameTickPacket};
/// # fn main() -> Result<(), Box<::std::error::Error>> {
/// let rlbot = rlbot::init()?;
/// rlbot.start_match(MatchSettings::simple_1v1("Hero", "Villain"))?;
///
/// let mut packets = rlbot.packeteer();
///
/// // Wait for the match to start. `packets.next_flatbuffer()` sleeps until the next
/// // packet is available, so this loop will not roast your CPU :)
/// while !packets.next()?.GameInfo.RoundActive {}
///
/// loop {
///     let packet = packets.next_flatbuffer()?;
///     let input: ControllerStateArgs = Default::default();
///     rlbot.update_player_input_flatbuffer(0, input)?;
/// }
/// # }
/// ```
///
/// See [`examples/simple`] for a complete example.
///
/// [`examples/simple`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/simple.rs
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

    /// Sends player input to RLBot using flatbuffers.
    pub fn update_player_input_flatbuffer(
        &self,
        player_index: i32,
        controller_state_args: flat::ControllerStateArgs,
    ) -> Result<(), RLBotError> {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
        let controller_state = flat::ControllerState::create(&mut builder, &controller_state_args);
        let player_input = flat::PlayerInput::create(
            &mut builder,
            &flat::PlayerInputArgs {
                playerIndex: player_index,
                controllerState: Some(controller_state),
            },
        );
        builder.finish(player_input, None);
        let status = (self.interface.update_player_input_flatbuffer)(
            builder.finished_data().as_ptr() as *mut c_void,
            builder.finished_data().len() as c_int,
        );
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

    /// Grabs the current [`GameTickPacket`](flat::GameTickPacket) from RLBot,
    /// if any. Consider using [`packeteer`](RLBot::packeteer) instead for
    /// a more convenient interface.
    pub fn update_live_data_packet_flatbuffer(&self) -> Option<flat::GameTickPacket> {
        let byte_buffer = (self.interface.update_live_data_packet_flatbuffer)();
        if byte_buffer.size == 0 {
            // game hasn't started yet
            None
        } else {
            let slice: &[u8] = unsafe {
                slice::from_raw_parts(byte_buffer.ptr as *const u8, byte_buffer.size as usize)
            };
            Some(flatbuffers::get_root::<flat::GameTickPacket>(&slice))
        }
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
