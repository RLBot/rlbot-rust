use dll::RLBotCoreInterface;
use error::RLBotError;
use ffi;
use flatbuffers;
use inject;
use packeteer::Packeteer;
use physicist::Physicist;
use rlbot_generated::rlbot::flat;
use std::cell::Cell;
use std::error::Error;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::slice;

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
/// # extern crate flatbuffers;
/// # extern crate rlbot;
/// # use rlbot::ffi::MatchSettings;
/// # use rlbot::flat;
/// #
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
///     let input_args: flat::PlayerInputArgs = Default::default();
///     let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
///     let player_input = flat::PlayerInput::create(&mut builder, &input_args);
///     builder.finish(player_input, None);
///     rlbot.update_player_input_flatbuffer(builder.finished_data())?;
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

/// The low-level interface to RLBot. All RLBot calls that are available can be
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

    /// Returns a [`Physicist`] object, for conveniently accessing physics
    /// ticks as they occur.
    pub fn physicist(&self) -> Physicist {
        Physicist::new(self)
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

    /// Sends player input to RLBot using flatbuffers. The buffer must be built
    /// from a [`flat::PlayerInput`]
    pub fn update_player_input_flatbuffer(
        &self,
        player_input_buffer: &[u8],
    ) -> Result<(), RLBotError> {
        let status = (self.interface.update_player_input_flatbuffer)(
            player_input_buffer.as_ptr() as *mut c_void,
            player_input_buffer.len() as c_int,
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

    /// Grabs the current [`flat::GameTickPacket`] from RLBot,
    /// if any. Consider using [`packeteer`](RLBot::packeteer) instead for
    /// a more convenient interface.
    pub fn update_live_data_packet_flatbuffer(&self) -> Option<flat::GameTickPacket> {
        let byte_buffer = (self.interface.update_live_data_packet_flatbuffer)();
        get_flatbuffer::<flat::GameTickPacket>(byte_buffer)
    }

    /// Grabs the current physics tick as a FlatBuffer table.
    pub fn update_rigid_body_tick_flatbuffer(&self) -> Option<flat::RigidBodyTick> {
        let byte_buffer = (self.interface.update_rigid_body_tick_flatbuffer)();
        get_flatbuffer::<flat::RigidBodyTick>(byte_buffer)
    }

    /// Grabs the current physics tick as a struct.
    pub fn update_rigid_body_tick(&self, tick: &mut ffi::RigidBodyTick) -> Result<(), RLBotError> {
        let status = (self.interface.update_rigid_body_tick)(tick);
        core_result(status)
    }

    /// Grabs the current [`ffi::FieldInfo`] from RLBot
    pub fn update_field_info(&self, field_info: &mut ffi::FieldInfo) -> Result<(), RLBotError> {
        let status = (self.interface.update_field_info)(field_info);
        core_result(status)
    }

    /// Grabs the current [`flat::FieldInfo`] from RLBot, if any
    pub fn update_field_info_flatbuffer(&self) -> Option<flat::FieldInfo> {
        let byte_buffer = (self.interface.update_field_info_flatbuffer)();
        get_flatbuffer::<flat::FieldInfo>(byte_buffer)
    }

    /// Sets the desired game state. The buffer must be built from a
    /// [`flat::DesiredGameState`]
    pub fn set_game_state(&self, desired_game_state_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.interface.set_game_state)(
            desired_game_state_buffer.as_ptr() as *mut c_void,
            desired_game_state_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Render a group of lines/text. The buffer must be built from a
    /// [`flat::RenderGroup`]
    pub fn render_group(&self, render_group_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.interface.render_group)(
            render_group_buffer.as_ptr() as *mut c_void,
            render_group_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Send a quickchat. The buffer must be built from a [`flat::QuickChat`]
    pub fn send_quick_chat(&self, quick_chat_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.interface.send_quick_chat)(
            quick_chat_buffer.as_ptr() as *mut c_void,
            quick_chat_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Tell RLBot to start a match.
    pub fn start_match(&self, match_settings: ffi::MatchSettings) -> Result<(), RLBotError> {
        let status = (self.interface.start_match)(match_settings, None, null_mut());
        core_result(status)
    }

    /// Gets the framework's current prediction of ball motion as a FlatBuffer
    /// table.
    ///
    /// Note that this method requires the framework's `BallPrediction.exe` to
    /// be running in the background.
    pub fn get_ball_prediction(&self) -> Option<flat::BallPrediction> {
        let byte_buffer = (self.interface.get_ball_prediction)();
        get_flatbuffer::<flat::BallPrediction>(byte_buffer)
    }

    /// Gets the framework's current prediction of ball motion as a struct.
    ///
    /// Note that this method requires the framework's `BallPrediction.exe` to
    /// be running in the background.
    pub fn get_ball_prediction_struct(
        &self,
        result: &mut ffi::BallPredictionPacket,
    ) -> Result<(), RLBotError> {
        let status = (self.interface.get_ball_prediction_struct)(result);
        core_result(status)
    }
}

fn core_result(status: ffi::RLBotCoreStatus) -> Result<(), RLBotError> {
    match status {
        ffi::RLBotCoreStatus::Success => Ok(()),
        _ => Err(RLBotError { status }),
    }
}

fn get_flatbuffer<'a, T: flatbuffers::Follow<'a> + 'a>(
    byte_buffer: ffi::ByteBuffer,
) -> Option<T::Inner> {
    if byte_buffer.size == 0 {
        return None;
    }

    let ptr = byte_buffer.ptr as *const u8;
    let size = byte_buffer.size as usize;
    let slice = unsafe { slice::from_raw_parts(ptr, size) };
    Some(flatbuffers::get_root::<T>(slice))
}
