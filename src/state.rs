//! This module contains data structures used by
//! [`RLBot::set_game_state_struct`] for state setting.

use crate::{
    rlbot_generated::rlbot::flat,
    state_convert::{Point3Into, Vector3Into},
};

/// Represents a vector in 3D space.
#[derive(Clone, Default)]
pub struct Vector3Partial {
    /// The X coordinate.
    pub x: Option<f32>,
    /// The Y coordinate.
    pub y: Option<f32>,
    /// The Z coordinate.
    pub z: Option<f32>,
    non_exhaustive: (),
}

impl Vector3Partial {
    /// Construts a new `Vector3Partial`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the X coordinate.
    pub fn x(mut self, x: f32) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the Y coordinate.
    pub fn y(mut self, y: f32) -> Self {
        self.y = Some(y);
        self
    }

    /// Sets the Z coordinate.
    pub fn z(mut self, z: f32) -> Self {
        self.z = Some(z);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::Vector3Partial<'a>> {
        let x = self.x.map(flat::Float::new);
        let y = self.y.map(flat::Float::new);
        let z = self.z.map(flat::Float::new);
        let args = flat::Vector3PartialArgs {
            x: x.as_ref(),
            y: y.as_ref(),
            z: z.as_ref(),
        };
        flat::Vector3Partial::create(builder, &args)
    }
}

/// A rotation in 3D space represented by Euler angles.
#[derive(Clone, Default)]
pub struct RotatorPartial {
    /// The pitch.
    pub pitch: Option<f32>,
    /// The yaw.
    pub yaw: Option<f32>,
    /// The roll.
    pub roll: Option<f32>,
    non_exhaustive: (),
}

impl RotatorPartial {
    /// Construts a new `RotatorPartial`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pitch.
    pub fn pitch(mut self, pitch: f32) -> Self {
        self.pitch = Some(pitch);
        self
    }

    /// Sets the yaw.
    pub fn yaw(mut self, yaw: f32) -> Self {
        self.yaw = Some(yaw);
        self
    }

    /// Sets the roll.
    pub fn roll(mut self, roll: f32) -> Self {
        self.roll = Some(roll);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::RotatorPartial<'a>> {
        let pitch = self.pitch.map(flat::Float::new);
        let yaw = self.yaw.map(flat::Float::new);
        let roll = self.roll.map(flat::Float::new);
        let args = flat::RotatorPartialArgs {
            pitch: pitch.as_ref(),
            yaw: yaw.as_ref(),
            roll: roll.as_ref(),
        };
        flat::RotatorPartial::create(builder, &args)
    }
}

/// Rigid-body state which can be set.
#[derive(Clone, Default)]
pub struct DesiredPhysics {
    /// The location of the rigid body.
    pub location: Option<Vector3Partial>,
    /// The rotation of the rigid body.
    pub rotation: Option<RotatorPartial>,
    /// The velocity of the rigid body.
    pub velocity: Option<Vector3Partial>,
    /// The angular velocity of the rigid body.
    pub angular_velocity: Option<Vector3Partial>,
    non_exhaustive: (),
}

impl DesiredPhysics {
    /// Constructs a new `DesiredPhysics`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the location.
    pub fn location(mut self, location: impl Point3Into<Vector3Partial>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Sets the rotation.
    pub fn rotation(mut self, rotation: impl Into<RotatorPartial>) -> Self {
        self.rotation = Some(rotation.into());
        self
    }

    /// Sets the velocity.
    pub fn velocity(mut self, velocity: impl Vector3Into<Vector3Partial>) -> Self {
        self.velocity = Some(velocity.into());
        self
    }

    /// Sets the angular velocity.
    pub fn angular_velocity(mut self, angular_velocity: impl Vector3Into<Vector3Partial>) -> Self {
        self.angular_velocity = Some(angular_velocity.into());
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::DesiredPhysics<'a>> {
        let args = flat::DesiredPhysicsArgs {
            location: self.location.as_ref().map(|x| x.serialize(builder)),
            rotation: self.rotation.as_ref().map(|x| x.serialize(builder)),
            velocity: self.velocity.as_ref().map(|x| x.serialize(builder)),
            angularVelocity: self.angular_velocity.as_ref().map(|x| x.serialize(builder)),
        };
        flat::DesiredPhysics::create(builder, &args)
    }
}

/// State which can be set on the ball.
#[derive(Clone, Default)]
pub struct DesiredBallState {
    /// The ball's physics.
    pub physics: Option<DesiredPhysics>,
    non_exhaustive: (),
}

impl DesiredBallState {
    /// Constructs a new `DesiredBallState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the physics.
    pub fn physics(mut self, physics: DesiredPhysics) -> Self {
        self.physics = Some(physics);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::DesiredBallState<'a>> {
        let args = flat::DesiredBallStateArgs {
            physics: self.physics.as_ref().map(|x| x.serialize(builder)),
        };
        flat::DesiredBallState::create(builder, &args)
    }
}

/// State which can be set on a car.
#[derive(Clone, Default)]
pub struct DesiredCarState {
    /// The car's physics.
    pub physics: Option<DesiredPhysics>,
    /// The amount of boost, from 0 to 100.
    pub boost_amount: Option<f32>,
    /// Whether the car has jumped using the jump button.
    pub jumped: Option<bool>,
    /// Whether the car has used its second jump.
    pub double_jumped: Option<bool>,
    non_exhaustive: (),
}

impl DesiredCarState {
    /// Constructs a new `DesiredCarState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the physics.
    pub fn physics(mut self, physics: DesiredPhysics) -> Self {
        self.physics = Some(physics);
        self
    }

    /// Sets the boost amount.
    pub fn boost_amount(mut self, boost_amount: f32) -> Self {
        self.boost_amount = Some(boost_amount);
        self
    }

    /// Sets whether the car has jumped.
    pub fn jumped(mut self, jumped: bool) -> Self {
        self.jumped = Some(jumped);
        self
    }

    /// Sets whether the car has used its double jump.
    pub fn double_jumped(mut self, double_jumped: bool) -> Self {
        self.double_jumped = Some(double_jumped);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::DesiredCarState<'a>> {
        let boost_amount = self.boost_amount.map(flat::Float::new);
        let jumped = self.jumped.map(flat::Bool::new);
        let double_jumped = self.double_jumped.map(flat::Bool::new);
        let args = flat::DesiredCarStateArgs {
            physics: self.physics.as_ref().map(|x| x.serialize(builder)),
            boostAmount: boost_amount.as_ref(),
            jumped: jumped.as_ref(),
            doubleJumped: double_jumped.as_ref(),
        };
        flat::DesiredCarState::create(builder, &args)
    }
}

/// State which can be set for a boost pickup.
#[derive(Clone, Default)]
pub struct DesiredBoostState {
    /// The amount of time until the boost pickup respawns.
    pub respawn_time: Option<f32>,
    non_exhaustive: (),
}

impl DesiredBoostState {
    /// Constructs a new `DesiredBoostState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets amount of time until the boost pickup respawns.
    pub fn respawn_time(mut self, respawn_time: f32) -> Self {
        self.respawn_time = Some(respawn_time);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::DesiredBoostState<'a>> {
        let respawn_time = self.respawn_time.map(flat::Float::new);
        let args = flat::DesiredBoostStateArgs {
            respawnTime: respawn_time.as_ref(),
        };
        flat::DesiredBoostState::create(builder, &args)
    }
}

/// State which can be set for a boost pickup.
#[derive(Clone, Default)]
pub struct DesiredGameInfoState {
    /// The gravity acceleration.
    pub world_gravity_z: Option<f32>,
    /// The game speed multiplier (`1.0` is normal speed).
    pub game_speed: Option<f32>,
    non_exhaustive: (),
}

impl DesiredGameInfoState {
    /// Constructs a new `DesiredGameInfoState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the gravity acceleration.
    pub fn world_gravity_z(mut self, world_gravity_z: f32) -> Self {
        self.world_gravity_z = Some(world_gravity_z);
        self
    }

    /// Sets the game speed multiplier (`1.0` is normal speed).
    pub fn game_speed(mut self, game_speed: f32) -> Self {
        self.game_speed = Some(game_speed);
        self
    }

    fn serialize<'a>(
        &self,
        builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<flat::DesiredGameInfoState<'a>> {
        let world_gravity_z = self.world_gravity_z.map(flat::Float::new);
        let game_speed = self.game_speed.map(flat::Float::new);
        let args = flat::DesiredGameInfoStateArgs {
            worldGravityZ: world_gravity_z.as_ref(),
            gameSpeed: game_speed.as_ref(),
        };
        flat::DesiredGameInfoState::create(builder, &args)
    }
}

/// The top-level struct containing all settable game state.
///
/// Pass an instance of this to
/// [`RLBot::set_game_state_struct`](crate::RLBot::set_game_state_struct) to
/// make the magic happen.
#[derive(Clone, Default)]
pub struct DesiredGameState {
    /// The state of the ball.
    pub ball_state: Option<DesiredBallState>,
    /// The state of each car.
    pub car_states: Vec<Option<DesiredCarState>>,
    /// The state of each boost pickup.
    pub boost_states: Vec<Option<DesiredBoostState>>,
    /// The state of the game environment.
    pub game_info_state: Option<DesiredGameInfoState>,
    non_exhaustive: (),
}

impl DesiredGameState {
    /// Constructs a new `DesiredGameState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the state of the ball.
    pub fn ball_state(mut self, ball_state: DesiredBallState) -> Self {
        self.ball_state = Some(ball_state);
        self
    }

    /// Sets the state of a car.
    pub fn car_state(mut self, index: usize, car_state: DesiredCarState) -> Self {
        if self.car_states.len() <= index {
            self.car_states.resize(index + 1, Default::default());
        }
        self.car_states[index] = Some(car_state);
        self
    }

    /// Sets the state of a boost pickup.
    pub fn boost_state(mut self, index: usize, boost_state: DesiredBoostState) -> Self {
        if self.boost_states.len() <= index {
            self.boost_states.resize(index + 1, Default::default());
        }
        self.boost_states[index] = Some(boost_state);
        self
    }

    /// Sets the state of the game environment.
    pub fn game_info_state(mut self, game_info_state: DesiredGameInfoState) -> Self {
        self.game_info_state = Some(game_info_state);
        self
    }

    pub(crate) fn serialize<'a>(&self) -> flatbuffers::FlatBufferBuilder<'a> {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

        let car_states = self
            .car_states
            .iter()
            .map(|cs| {
                cs.as_ref()
                    .unwrap_or(&DesiredCarState::new())
                    .serialize(&mut builder)
            })
            .collect::<Vec<_>>();
        let boost_states = self
            .boost_states
            .iter()
            .map(|s| {
                s.as_ref()
                    .unwrap_or(&DesiredBoostState::new())
                    .serialize(&mut builder)
            })
            .collect::<Vec<_>>();
        let args = flat::DesiredGameStateArgs {
            ballState: self.ball_state.as_ref().map(|x| x.serialize(&mut builder)),
            carStates: Some(builder.create_vector(&car_states)),
            boostStates: Some(builder.create_vector(&boost_states)),
            gameInfoState: self
                .game_info_state
                .as_ref()
                .map(|x| x.serialize(&mut builder)),
            consoleCommands: None,
        };
        let root = flat::DesiredGameState::create(&mut builder, &args);

        builder.finish(root, None);
        builder
    }
}

#[cfg(feature = "nalgebra")]
#[cfg(test)]
mod tests {
    use crate::state;
    use na::{Point3, Vector3};

    #[test]
    fn test_nalgebra_arguments() {
        let _ = state::DesiredPhysics::new()
            .location(Point3::origin())
            .velocity(Vector3::zeros())
            .angular_velocity(Vector3::zeros());
    }
}
