#![allow(missing_docs)]

use crate::flat;
pub use flat::TileState;
use smallvec::SmallVec;

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
    /// true if you want to use the current item
    pub use_item: bool,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Expresses the rotation state of an object in Euler angles, with values in
/// radians.
#[derive(Default)]
pub struct Rotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

/// Expresses the rotation state of an object.
/// Learn about quaternions here: https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
/// You can tinker with them here to build an intuition: https://quaternions.online/
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Represents contact between a car and the ball.
pub struct Touch {
    /// The name of the player involved with the touch.
    pub player_name: String,
    /// Seconds that had elapsed in the game when the touch occurred.
    pub game_seconds: f32,
    /// The point of contact for the touch.
    pub location: Vector3,
    /// The direction of the touch.
    pub normal: Vector3,
    /// The Team which the touch belongs to, 0 for blue 1 for orange.
    pub team: i32,
    pub(crate) _non_exhaustive: (),
}

pub struct ScoreInfo {
    pub score: i32,
    pub goals: i32,
    pub own_goals: i32,
    pub assists: i32,
    pub saves: i32,
    pub shots: i32,
    pub demolitions: i32,
    pub(crate) _non_exhaustive: (),
}

pub struct Physics {
    pub location: Vector3,
    pub rotation: Rotator,
    pub velocity: Vector3,
    pub angular_velocity: Vector3,
    pub(crate) _non_exhaustive: (),
}

pub struct PlayerInfo {
    pub physics: Physics,
    pub score_info: ScoreInfo,
    pub is_demolished: bool,
    /// True if your wheels are on the ground, the wall, or the ceiling. False
    /// if you're midair or turtling.
    pub has_wheel_contact: bool,
    pub is_supersonic: bool,
    pub is_bot: bool,
    /// True if the player has jumped. Falling off the ceiling / driving off the
    /// goal post does not count.
    pub jumped: bool,
    ///  True if player has double jumped. False does not mean you have a jump
    /// remaining, because the  aerial timer can run out, and that doesn't
    /// affect this flag.
    pub double_jumped: bool,
    pub name: String,
    pub team: i32,
    pub boost: i32,
    pub(crate) _non_exhaustive: (),
}

pub struct DropshotBallInfo {
    pub absorbed_force: f32,
    pub damage_index: i32,
    pub force_accum_recent: f32,
    pub(crate) _non_exhaustive: (),
}

pub struct BallInfo {
    pub physics: Physics,
    pub latest_touch: Option<Touch>,
    pub dropshot_info: Option<DropshotBallInfo>,
    pub(crate) _non_exhaustive: (),
}

pub struct BoostPadState {
    /// True if the boost can be picked up
    pub is_active: bool,
    /// The number of seconds since the boost has been picked up, or 0.0 if the
    /// boost is active.
    pub timer: f32,
    pub(crate) _non_exhaustive: (),
}

pub struct DropshotTile {
    pub tile_state: TileState,
    pub(crate) _non_exhaustive: (),
}

pub struct GameInfo {
    pub seconds_elapsed: f32,
    pub game_time_remaining: f32,
    pub is_overtime: bool,
    pub is_unlimited_time: bool,
    /// True when cars are allowed to move, and during the pause menu. False
    /// during replays.
    pub is_round_active: bool,
    /// True when the clock is paused due to kickoff, but false during kickoff
    /// countdown. In other words, it is true while cars can move during
    /// kickoff. Note that if both players sit still, game clock start and this
    /// will become false.
    pub is_kickoff_pause: bool,
    /// Turns true after final replay, the moment the 'winner' screen appears.
    /// Remains true during next match countdown. Turns false again the
    /// moment the 'choose team' screen appears.
    pub is_match_ended: bool,
    pub world_gravity_z: f32,
    /// Game speed multiplier, 1.0 is regular game speed.
    pub game_speed: f32,
    pub(crate) _non_exhaustive: (),
}

pub struct TeamInfo {
    pub team_index: i32,
    /// number of goals scored.
    pub score: i32,
    pub(crate) _non_exhaustive: (),
}

pub struct GameTickPacket {
    pub players: SmallVec<[PlayerInfo; 4]>,
    // Until generic const lands in Rust, we're limited to array sizes handpicked by smallvec.
    // Ideally this would be 34:
    pub boost_pad_states: SmallVec<[BoostPadState; 36]>,
    /// Info about the ball. This is usually present, but can be `None` at these
    /// times:
    ///
    /// - Briefly at the start of a goal replay
    /// - During PodiumSpotlight when the winning team is (ideally) celebrating.
    /// - Possibly at other times not during normal gameplay.
    pub ball: Option<BallInfo>,
    pub game_info: GameInfo,
    /// The state of each dropshot tile. This will be `None` if the current game
    /// is not a Dropshot game.
    // Ideally this would be 140:
    pub tile_information: Option<SmallVec<[DropshotTile; 256]>>,
    pub teams: SmallVec<[TeamInfo; 2]>,
    pub(crate) _non_exhaustive: (),
}

pub struct GoalInfo {
    pub team_num: i32,
    pub location: Vector3,
    pub direction: Vector3,
    pub(crate) _non_exhaustive: (),
}

pub struct BoostPad {
    pub location: Vector3,
    pub full_boost: bool,
    pub(crate) _non_exhaustive: (),
}

pub struct FieldInfo {
    pub boost_pads: SmallVec<[BoostPad; 64]>,
    pub goals: SmallVec<[GoalInfo; 256]>,
    pub(crate) _non_exhaustive: (),
}

pub struct RigidBodyState {
    pub frame: i32,
    pub location: Vector3,
    pub rotation: Quaternion,
    pub velocity: Vector3,
    pub angular_velocity: Vector3,
    pub(crate) _non_exhaustive: (),
}

pub struct PlayerRigidBodyState {
    pub state: RigidBodyState,
    pub input: ControllerState,
    pub(crate) _non_exhaustive: (),
}

pub struct BallRigidBodyState {
    pub state: Option<RigidBodyState>,
    pub(crate) _non_exhaustive: (),
}

pub struct RigidBodyTick {
    pub ball: Option<BallRigidBodyState>,
    pub players: SmallVec<[PlayerRigidBodyState; 10]>,
    pub(crate) _non_exhaustive: (),
}

pub struct PredictionSlice {
    pub game_seconds: f32,
    pub physics: Physics,
    pub(crate) _non_exhaustive: (),
}

pub struct BallPrediction {
    pub slices: SmallVec<[PredictionSlice; 512]>,
    pub(crate) _non_exhaustive: (),
}

pub(crate) fn build_update_player_input(
    player_index: i32,
    controller_state: &ControllerState,
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
        useItem: controller_state.use_item,
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
