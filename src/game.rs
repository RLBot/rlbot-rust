use crate::flat;

/// The inputs that control a car.
#[derive(Default)]
pub struct ControllerState {
    /// -1 for full reverse, 1 for full forward
    pub throttle: f32,
    /// -1 for full left, 1 for full right
    pub steer: f32,
    /// -1 for nose down, 1 for nose up
    pub pitch: f32,
    /// -1 for full left, 1 for full right
    pub yaw: f32,
    /// -1 for roll left, 1 for roll right
    pub roll: f32,
    /// true if you want to press the jump button
    pub jump: bool,
    /// true if you want to press the boost button
    pub boost: bool,
    /// true if you want to press the handbrake button
    pub handbrake: bool,
}

pub(crate) fn build_update_player_input(
    player_index: i32,
    controller_state: ControllerState,
) -> flatbuffers::FlatBufferBuilder<'static> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(64);

    let controller_state_args = flat::ControllerStateArgs {
        throttle: controller_state.throttle,
        steer: controller_state.steer,
        pitch: controller_state.pitch,
        yaw: controller_state.yaw,
        roll: controller_state.roll,
        jump: controller_state.jump,
        boost: controller_state.boost,
        handbrake: controller_state.handbrake,
    };
    let controller_state = flat::ControllerState::create(&mut builder, &controller_state_args);

    let args = flat::PlayerInputArgs {
        playerIndex: player_index,
        controllerState: Some(controller_state),
    };
    let player_input = flat::PlayerInput::create(&mut builder, &args);

    builder.finish(player_input, None);
    builder
}
