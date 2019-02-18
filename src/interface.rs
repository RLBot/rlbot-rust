use crate::{dll::RLBotCoreInterface, error::RLBotError, ffi, flat};
use std::{
    os::raw::{c_int, c_void},
    ptr::null_mut,
    slice,
};

pub struct RLBotInterface {
    dll: RLBotCoreInterface,
}

impl RLBotInterface {
    pub fn new(dll: RLBotCoreInterface) -> Self {
        Self { dll }
    }

    /// Grabs the current [`flat::FieldInfo`] from RLBot, if any
    pub fn update_field_info_flatbuffer<'fb>(&self) -> Option<flat::FieldInfo<'fb>> {
        let byte_buffer = (self.dll.update_field_info_flatbuffer)();
        get_flatbuffer::<flat::FieldInfo<'_>>(byte_buffer)
    }

    /// Grabs the current [`ffi::FieldInfo`] from RLBot
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn update_field_info(&self, field_info: &mut ffi::FieldInfo) -> Result<(), RLBotError> {
        let status = (self.dll.update_field_info)(field_info);
        core_result(status)
    }

    /// Grabs the current [`flat::GameTickPacket`] from RLBot,
    /// if any. Consider using [`packeteer`](RLBot::packeteer) instead for
    /// a more convenient interface.
    pub fn update_live_data_packet_flatbuffer<'fb>(&self) -> Option<flat::GameTickPacket<'fb>> {
        let byte_buffer = (self.dll.update_live_data_packet_flatbuffer)();
        get_flatbuffer::<flat::GameTickPacket<'_>>(byte_buffer)
    }

    /// Grabs the current [`LiveDataPacket`](ffi::LiveDataPacket) from RLBot.
    /// Consider using [`packeteer`](RLBot::packeteer) instead for a more
    /// convenient interface.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn update_live_data_packet(
        &self,
        packet: &mut ffi::LiveDataPacket,
    ) -> Result<(), RLBotError> {
        let status = (self.dll.update_live_data_packet)(packet);
        core_result(status)
    }

    /// Grabs the current physics tick as a FlatBuffer table.
    pub fn update_rigid_body_tick_flatbuffer<'fb>(&self) -> Option<flat::RigidBodyTick<'fb>> {
        let byte_buffer = (self.dll.update_rigid_body_tick_flatbuffer)();
        get_flatbuffer::<flat::RigidBodyTick<'_>>(byte_buffer)
    }

    /// Grabs the current physics tick as a struct.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn update_rigid_body_tick(&self, tick: &mut ffi::RigidBodyTick) -> Result<(), RLBotError> {
        let status = (self.dll.update_rigid_body_tick)(tick);
        core_result(status)
    }

    /// Sets the desired game state. The buffer must be built from a
    /// [`flat::DesiredGameState`]
    pub fn set_game_state(&self, desired_game_state_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.dll.set_game_state)(
            desired_game_state_buffer.as_ptr() as *mut c_void,
            desired_game_state_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Tell RLBot to start a match.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn start_match(&self, match_settings: ffi::MatchSettings) -> Result<(), RLBotError> {
        let status = (self.dll.start_match)(match_settings, None, null_mut());
        core_result(status)
    }

    /// Tell RLBot to start a match. The buffer must be built from a
    /// [`flat::QuickChat`].
    pub fn start_match_flatbuffer(&self, match_settings_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.dll.start_match_flatbuffer)(
            match_settings_buffer.as_ptr() as *mut c_void,
            match_settings_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Send a quickchat. The buffer must be built from a [`flat::QuickChat`]
    pub fn send_quick_chat(&self, quick_chat_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.dll.send_quick_chat)(
            quick_chat_buffer.as_ptr() as *mut c_void,
            quick_chat_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Sends player input to RLBot.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn update_player_input(
        &self,
        player_input: ffi::PlayerInput,
        player_index: c_int,
    ) -> Result<(), RLBotError> {
        let status = (self.dll.update_player_input)(player_input, player_index);
        core_result(status)
    }

    /// Sends player input to RLBot using flatbuffers. The buffer must be built
    /// from a [`flat::PlayerInput`]
    pub fn update_player_input_flatbuffer(
        &self,
        player_input_buffer: &[u8],
    ) -> Result<(), RLBotError> {
        let status = (self.dll.update_player_input_flatbuffer)(
            player_input_buffer.as_ptr() as *mut c_void,
            player_input_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Render a group of lines/text. The buffer must be built from a
    /// [`flat::RenderGroup`]
    pub fn render_group(&self, render_group_buffer: &[u8]) -> Result<(), RLBotError> {
        let status = (self.dll.render_group)(
            render_group_buffer.as_ptr() as *mut c_void,
            render_group_buffer.len() as c_int,
        );
        core_result(status)
    }

    /// Gets the framework's current prediction of ball motion as a FlatBuffer
    /// table.
    ///
    /// Note that this method requires the framework's `BallPrediction.exe` to
    /// be running in the background.
    pub fn get_ball_prediction<'fb>(&self) -> Option<flat::BallPrediction<'fb>> {
        let byte_buffer = (self.dll.get_ball_prediction)();
        get_flatbuffer::<flat::BallPrediction<'_>>(byte_buffer)
    }

    /// Gets the framework's current prediction of ball motion as a struct.
    ///
    /// Note that this method requires the framework's `BallPrediction.exe` to
    /// be running in the background.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    pub fn get_ball_prediction_struct(
        &self,
        result: &mut ffi::BallPredictionPacket,
    ) -> Result<(), RLBotError> {
        let status = (self.dll.get_ball_prediction_struct)(result);
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
    // TODO: don't leak the buffer
    let slice = unsafe { slice::from_raw_parts(ptr, size) };
    Some(flatbuffers::get_root::<T>(slice))
}

#[cfg(test)]
mod tests {
    use crate::{ffi, interface::RLBotInterface};
    use std::{error::Error, mem};

    #[test]
    #[ignore(note = "compile-only test")]
    fn game_data_is_send() -> Result<(), Box<dyn Error>> {
        fn assert_send<T: Send + 'static>(_: T) {}

        assert_send(ffi::LiveDataPacket::default());
        assert_send(ffi::RigidBodyTick::default());
        assert_send(ffi::FieldInfo::default());
        assert_send(ffi::BallPredictionPacket::default());

        let interface: RLBotInterface = unsafe { mem::uninitialized() };
        assert_send(interface.update_live_data_packet_flatbuffer());
        assert_send(interface.update_rigid_body_tick_flatbuffer());
        assert_send(interface.update_field_info_flatbuffer());
        assert_send(interface.get_ball_prediction());
        Ok(())
    }
}
