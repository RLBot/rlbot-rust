use crate::{flat, game::*, utils::flat_vector_iter};

impl From<flat::ControllerState<'_>> for ControllerState {
    fn from(state: flat::ControllerState<'_>) -> Self {
        Self {
            throttle: state.throttle(),
            steer: state.steer(),
            pitch: state.pitch(),
            yaw: state.yaw(),
            roll: state.roll(),
            jump: state.jump(),
            boost: state.boost(),
            handbrake: state.handbrake(),
            use_item: state.useItem(),
        }
    }
}

impl From<flat::GameTickPacket<'_>> for GameTickPacket {
    fn from(packet: flat::GameTickPacket<'_>) -> Self {
        Self {
            players: flat_vector_iter(packet.players().unwrap())
                .map(PlayerInfo::from)
                .collect(),
            boost_pad_states: flat_vector_iter(packet.boostPadStates().unwrap())
                .map(BoostPadState::from)
                .collect(),
            ball: packet.ball().map(BallInfo::from),
            game_info: packet.gameInfo().unwrap().into(),
            tile_information: packet
                .tileInformation()
                .map(|ti| flat_vector_iter(ti).map(DropshotTile::from).collect()),
            teams: flat_vector_iter(packet.teams().unwrap())
                .map(TeamInfo::from)
                .collect(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::PlayerInfo<'_>> for PlayerInfo {
    fn from(info: flat::PlayerInfo<'_>) -> Self {
        Self {
            physics: info.physics().unwrap().into(),
            score_info: info.scoreInfo().unwrap().into(),
            is_demolished: info.isDemolished(),
            has_wheel_contact: info.hasWheelContact(),
            is_supersonic: info.isSupersonic(),
            is_bot: info.isBot(),
            jumped: info.jumped(),
            double_jumped: info.doubleJumped(),
            name: info.name().unwrap().to_string(),
            team: info.team(),
            boost: info.boost(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::BoostPadState<'_>> for BoostPadState {
    fn from(state: flat::BoostPadState<'_>) -> Self {
        Self {
            is_active: state.isActive(),
            timer: state.timer(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::BallInfo<'_>> for BallInfo {
    fn from(info: flat::BallInfo<'_>) -> Self {
        Self {
            physics: info.physics().unwrap().into(),
            latest_touch: info.latestTouch().map(Touch::from),
            dropshot_info: info.dropShotInfo().map(DropshotBallInfo::from),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::Physics<'_>> for Physics {
    fn from(physics: flat::Physics<'_>) -> Self {
        Self {
            location: physics.location().unwrap().into(),
            rotation: physics.rotation().unwrap().into(),
            velocity: physics.velocity().unwrap().into(),
            angular_velocity: physics.angularVelocity().unwrap().into(),
            _non_exhaustive: (),
        }
    }
}

impl From<&flat::Vector3> for Vector3 {
    fn from(vector3: &flat::Vector3) -> Self {
        Self {
            x: vector3.x(),
            y: vector3.y(),
            z: vector3.z(),
        }
    }
}

impl From<&flat::Rotator> for Rotator {
    fn from(rotator: &flat::Rotator) -> Self {
        Self {
            pitch: rotator.pitch(),
            yaw: rotator.yaw(),
            roll: rotator.roll(),
        }
    }
}

impl From<&flat::Quaternion> for Quaternion {
    fn from(quaternion: &flat::Quaternion) -> Self {
        Self {
            x: quaternion.x(),
            y: quaternion.y(),
            z: quaternion.z(),
            w: quaternion.w(),
        }
    }
}

impl From<flat::Touch<'_>> for Touch {
    fn from(touch: flat::Touch<'_>) -> Self {
        Self {
            player_name: touch.playerName().unwrap().to_string(),
            game_seconds: touch.gameSeconds(),
            location: touch.location().unwrap().into(),
            normal: touch.normal().unwrap().into(),
            team: touch.team(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::DropShotBallInfo<'_>> for DropshotBallInfo {
    fn from(info: flat::DropShotBallInfo<'_>) -> Self {
        Self {
            absorbed_force: info.absorbedForce(),
            damage_index: info.damageIndex(),
            force_accum_recent: info.forceAccumRecent(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::GameInfo<'_>> for GameInfo {
    fn from(info: flat::GameInfo<'_>) -> Self {
        Self {
            seconds_elapsed: info.secondsElapsed(),
            game_time_remaining: info.gameTimeRemaining(),
            is_overtime: info.isOvertime(),
            is_unlimited_time: info.isUnlimitedTime(),
            is_round_active: info.isRoundActive(),
            is_kickoff_pause: info.isKickoffPause(),
            is_match_ended: info.isMatchEnded(),
            world_gravity_z: info.worldGravityZ(),
            game_speed: info.gameSpeed(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::DropshotTile<'_>> for DropshotTile {
    fn from(tile: flat::DropshotTile<'_>) -> Self {
        Self {
            tile_state: tile.tileState(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::TeamInfo<'_>> for TeamInfo {
    fn from(info: flat::TeamInfo<'_>) -> Self {
        Self {
            team_index: info.teamIndex(),
            score: info.score(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::ScoreInfo<'_>> for ScoreInfo {
    fn from(info: flat::ScoreInfo<'_>) -> Self {
        Self {
            score: info.score(),
            goals: info.goals(),
            own_goals: info.ownGoals(),
            assists: info.assists(),
            saves: info.saves(),
            shots: info.shots(),
            demolitions: info.demolitions(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::GoalInfo<'_>> for GoalInfo {
    fn from(goal_info: flat::GoalInfo<'_>) -> Self {
        Self {
            team_num: goal_info.teamNum(),
            location: goal_info.location().unwrap().into(),
            direction: goal_info.direction().unwrap().into(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::BoostPad<'_>> for BoostPad {
    fn from(boost_pad: flat::BoostPad<'_>) -> Self {
        Self {
            location: boost_pad.location().unwrap().into(),
            full_boost: boost_pad.isFullBoost(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::FieldInfo<'_>> for FieldInfo {
    fn from(info: flat::FieldInfo<'_>) -> Self {
        Self {
            boost_pads: flat_vector_iter(info.boostPads().unwrap())
                .map(BoostPad::from)
                .collect(),
            goals: flat_vector_iter(info.goals().unwrap())
                .map(GoalInfo::from)
                .collect(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::RigidBodyState<'_>> for RigidBodyState {
    fn from(state: flat::RigidBodyState<'_>) -> Self {
        Self {
            frame: state.frame(),
            location: state.location().unwrap().into(),
            rotation: state.rotation().unwrap().into(),
            velocity: state.velocity().unwrap().into(),
            angular_velocity: state.angularVelocity().unwrap().into(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::PlayerRigidBodyState<'_>> for PlayerRigidBodyState {
    fn from(state: flat::PlayerRigidBodyState<'_>) -> Self {
        Self {
            state: state.state().unwrap().into(),
            input: state.input().unwrap().into(),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::BallRigidBodyState<'_>> for BallRigidBodyState {
    fn from(state: flat::BallRigidBodyState<'_>) -> Self {
        Self {
            state: state.state().map(RigidBodyState::from),
            _non_exhaustive: (),
        }
    }
}

impl From<flat::RigidBodyTick<'_>> for RigidBodyTick {
    fn from(tick: flat::RigidBodyTick<'_>) -> Self {
        Self {
            ball: tick.ball().map(BallRigidBodyState::from),
            players: flat_vector_iter(tick.players().unwrap())
                .map(PlayerRigidBodyState::from)
                .collect(),
            _non_exhaustive: (),
        }
    }
}
