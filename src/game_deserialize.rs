use crate::{flat, game::*, utils::flat_vector_iter};

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

#[allow(unused)]
#[deprecated(note = "Use The From trait implementation instead")]
pub fn deserialize_game_tick_packet(packet: flat::GameTickPacket<'_>) -> GameTickPacket {
    GameTickPacket {
        players: flat_vector_iter(packet.players().unwrap())
            .map(deserialize_player_info)
            .collect(),
        boost_pad_states: flat_vector_iter(packet.boostPadStates().unwrap())
            .map(deserialize_boost_pad_state)
            .collect(),
        ball: packet.ball().map(deserialize_ball_info),
        game_info: deserialize_game_info(packet.gameInfo().unwrap()),
        tile_information: packet.tileInformation().map(|ti| {
            flat_vector_iter(ti)
                .map(deserialize_dropshot_tile)
                .collect()
        }),
        teams: flat_vector_iter(packet.teams().unwrap())
            .map(deserialize_team_info)
            .collect(),
        _non_exhaustive: (),
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

fn deserialize_player_info(info: flat::PlayerInfo<'_>) -> PlayerInfo {
    PlayerInfo {
        physics: deserialize_physics(info.physics().unwrap()),
        score_info: deserialize_score_info(info.scoreInfo().unwrap()),
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

impl From<flat::BoostPadState<'_>> for BoostPadState {
    fn from(state: flat::BoostPadState<'_>) -> Self {
        Self {
            is_active: state.isActive(),
            timer: state.timer(),
            _non_exhaustive: (),
        }
    }
}

fn deserialize_boost_pad_state(state: flat::BoostPadState<'_>) -> BoostPadState {
    BoostPadState {
        is_active: state.isActive(),
        timer: state.timer(),
        _non_exhaustive: (),
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

fn deserialize_ball_info(info: flat::BallInfo<'_>) -> BallInfo {
    BallInfo {
        physics: deserialize_physics(info.physics().unwrap()),
        latest_touch: info.latestTouch().map(deserialize_touch),
        dropshot_info: info.dropShotInfo().map(deserialize_dropshot_ball_info),
        _non_exhaustive: (),
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

fn deserialize_physics(physics: flat::Physics<'_>) -> Physics {
    Physics {
        location: deserialize_vector3(physics.location().unwrap()),
        rotation: deserialize_rotator(physics.rotation().unwrap()),
        velocity: deserialize_vector3(physics.velocity().unwrap()),
        angular_velocity: deserialize_vector3(physics.angularVelocity().unwrap()),
        _non_exhaustive: (),
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

fn deserialize_vector3(vector3: &flat::Vector3) -> Vector3 {
    Vector3 {
        x: vector3.x(),
        y: vector3.y(),
        z: vector3.z(),
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

fn deserialize_rotator(rotator: &flat::Rotator) -> Rotator {
    Rotator {
        pitch: rotator.pitch(),
        yaw: rotator.yaw(),
        roll: rotator.roll(),
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

fn deserialize_touch(touch: flat::Touch<'_>) -> Touch {
    Touch {
        player_name: touch.playerName().unwrap().to_string(),
        game_seconds: touch.gameSeconds(),
        location: deserialize_vector3(touch.location().unwrap()),
        normal: deserialize_vector3(touch.normal().unwrap()),
        team: touch.team(),
        _non_exhaustive: (),
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

fn deserialize_dropshot_ball_info(info: flat::DropShotBallInfo<'_>) -> DropshotBallInfo {
    DropshotBallInfo {
        absorbed_force: info.absorbedForce(),
        damage_index: info.damageIndex(),
        force_accum_recent: info.forceAccumRecent(),
        _non_exhaustive: (),
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

fn deserialize_game_info(info: flat::GameInfo<'_>) -> GameInfo {
    GameInfo {
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

impl From<flat::DropshotTile<'_>> for DropshotTile {
    fn from(tile: flat::DropshotTile<'_>) -> Self {
        Self {
            tile_state: tile.tileState(),
            _non_exhaustive: (),
        }
    }
}

fn deserialize_dropshot_tile(tile: flat::DropshotTile<'_>) -> DropshotTile {
    DropshotTile {
        tile_state: tile.tileState(),
        _non_exhaustive: (),
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

fn deserialize_team_info(info: flat::TeamInfo<'_>) -> TeamInfo {
    TeamInfo {
        team_index: info.teamIndex(),
        score: info.score(),
        _non_exhaustive: (),
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

fn deserialize_score_info(info: flat::ScoreInfo<'_>) -> ScoreInfo {
    ScoreInfo {
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
