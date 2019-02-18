use crate::{flat, game::*, utils::flat_vector_iter};

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

fn deserialize_boost_pad_state(state: flat::BoostPadState<'_>) -> BoostPadState {
    BoostPadState {
        is_active: state.isActive(),
        timer: state.timer(),
        _non_exhaustive: (),
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

fn deserialize_physics(physics: flat::Physics<'_>) -> Physics {
    Physics {
        location: deserialize_vector3(physics.location().unwrap()),
        rotation: deserialize_rotator(physics.rotation().unwrap()),
        velocity: deserialize_vector3(physics.velocity().unwrap()),
        angular_velocity: deserialize_vector3(physics.angularVelocity().unwrap()),
        _non_exhaustive: (),
    }
}

fn deserialize_vector3(vector3: &flat::Vector3) -> Vector3 {
    Vector3 {
        x: vector3.x(),
        y: vector3.y(),
        z: vector3.z(),
    }
}

fn deserialize_rotator(rotator: &flat::Rotator) -> Rotator {
    Rotator {
        pitch: rotator.pitch(),
        yaw: rotator.yaw(),
        roll: rotator.roll(),
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

fn deserialize_dropshot_ball_info(info: flat::DropShotBallInfo<'_>) -> DropshotBallInfo {
    DropshotBallInfo {
        absorbed_force: info.absorbedForce(),
        damage_index: info.damageIndex(),
        force_accum_recent: info.forceAccumRecent(),
        _non_exhaustive: (),
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

fn deserialize_dropshot_tile(tile: flat::DropshotTile<'_>) -> DropshotTile {
    DropshotTile {
        tile_state: tile.tileState(),
        _non_exhaustive: (),
    }
}

fn deserialize_team_info(info: flat::TeamInfo<'_>) -> TeamInfo {
    TeamInfo {
        team_index: info.teamIndex(),
        score: info.score(),
        _non_exhaustive: (),
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
