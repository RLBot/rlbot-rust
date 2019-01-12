use crate::{
    dll::RLBotCoreInterface, error::RLBotError, ffi, inject, packeteer::Packeteer,
    physicist::Physicist, render::RenderGroup, rlbot_generated::rlbot::flat, state,
};
use std::{
    cell::Cell,
    error::Error,
    marker::PhantomData,
    os::raw::{c_int, c_void},
    path::PathBuf,
    ptr::null_mut,
    slice,
};

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
/// rlbot.start_match(MatchSettings::rlbot_vs_allstar("Hero", "Villain"))?;
/// rlbot.wait_for_match_start()?;
///
/// let mut packets = rlbot.packeteer();
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
pub fn init_with_options(options: InitOptions) -> Result<RLBot, Box<dyn Error>> {
    let rlbot_dll_directory = options.rlbot_dll_directory.as_ref().map(|p| p.as_path());

    inject::inject_dll(rlbot_dll_directory)?;

    let interface = RLBotCoreInterface::load(rlbot_dll_directory)?;
    interface.wait_for_initialized()?;

    Ok(RLBot::new(interface))
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
    pub fn packeteer(&self) -> Packeteer<'_> {
        Packeteer::new(self)
    }

    /// Returns a [`Physicist`] object, for conveniently accessing physics
    /// ticks as they occur.
    pub fn physicist(&self) -> Physicist<'_> {
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
    pub fn update_live_data_packet_flatbuffer<'fb>(&self) -> Option<flat::GameTickPacket<'fb>> {
        let byte_buffer = (self.interface.update_live_data_packet_flatbuffer)();
        get_flatbuffer::<flat::GameTickPacket<'_>>(byte_buffer)
    }

    /// Grabs the current physics tick as a FlatBuffer table.
    pub fn update_rigid_body_tick_flatbuffer<'fb>(&self) -> Option<flat::RigidBodyTick<'fb>> {
        let byte_buffer = (self.interface.update_rigid_body_tick_flatbuffer)();
        get_flatbuffer::<flat::RigidBodyTick<'_>>(byte_buffer)
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
    pub fn update_field_info_flatbuffer<'fb>(&self) -> Option<flat::FieldInfo<'fb>> {
        let byte_buffer = (self.interface.update_field_info_flatbuffer)();
        get_flatbuffer::<flat::FieldInfo<'_>>(byte_buffer)
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

    /// Sets the game state.
    pub fn set_game_state_struct(
        &self,
        desired_game_state: state::DesiredGameState,
    ) -> Result<(), RLBotError> {
        let buffer = desired_game_state.serialize();
        self.set_game_state(buffer.finished_data())
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

    /// Spin-waits until a match is active.
    ///
    /// Call `start_match` before calling this method.
    pub fn wait_for_match_start(&self) -> Result<(), Box<dyn Error>> {
        let mut packets = self.packeteer();
        let mut count = 0;

        // Sometimes we get a few stray ticks from a previous game while the next game
        // is loading. Wait for RoundActive to stabilize before trusting it.
        while count < 5 {
            if packets.next()?.GameInfo.RoundActive {
                count += 1;
            } else {
                count = 0;
            }
        }
        Ok(())
    }

    /// Begin drawing to the screen.
    ///
    /// The ID identifies a group. Multiple groups can exist and be updated
    /// independently. Drawings will remain on screen until they are replaced
    /// by a group with the same ID.
    ///
    /// A group can be cleared from the screen by rendering an empty group.
    ///
    /// See [`RenderGroup`] for more info.
    pub fn begin_render_group(&self, id: i32) -> RenderGroup<'_> {
        RenderGroup::new(self, id)
    }

    /// Gets the framework's current prediction of ball motion as a FlatBuffer
    /// table.
    ///
    /// Note that this method requires the framework's `BallPrediction.exe` to
    /// be running in the background.
    pub fn get_ball_prediction<'fb>(&self) -> Option<flat::BallPrediction<'fb>> {
        let byte_buffer = (self.interface.get_ball_prediction)();
        get_flatbuffer::<flat::BallPrediction<'_>>(byte_buffer)
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

#[cfg(test)]
mod tests {
    use crate::{ffi, rlbot::RLBot};
    use std::{error::Error, mem};

    #[test]
    #[ignore(note = "compile-only test")]
    fn game_data_is_send() -> Result<(), Box<dyn Error>> {
        fn assert_send<T: Send + 'static>(_: T) {}

        assert_send(ffi::LiveDataPacket::default());
        assert_send(ffi::RigidBodyTick::default());
        assert_send(ffi::FieldInfo::default());
        assert_send(ffi::BallPredictionPacket::default());

        let rlbot: RLBot = unsafe { mem::uninitialized() };
        assert_send(rlbot.physicist().next_flat()?);
        assert_send(rlbot.packeteer().next()?);
        assert_send(rlbot.packeteer().next_flatbuffer()?);
        assert_send(rlbot.update_live_data_packet_flatbuffer());
        assert_send(rlbot.update_rigid_body_tick_flatbuffer());
        assert_send(rlbot.update_field_info_flatbuffer());
        assert_send(rlbot.get_ball_prediction());
        Ok(())
    }
}
