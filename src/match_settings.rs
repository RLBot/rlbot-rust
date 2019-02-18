#![allow(missing_docs)]

pub use crate::flat::{
    BallBouncinessOption, BallMaxSpeedOption, BallSizeOption, BallTypeOption, BallWeightOption,
    BoostOption, DemolishOption, GameMap, GameMode, GameSpeedOption, GravityOption, MatchLength,
    MaxScore, OvertimeOption, RespawnTimeOption, RumbleOption, SeriesLengthOption,
};
use crate::{flat, rlbot_generated::rlbot::flat::BoostStrengthOption};
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

/// A psyonix bot, e.g. All Star bot
#[derive(Clone, Default)]
pub struct PsyonixBotPlayer {
    /// The skill of the bot, from 0.0 (Rookie) to 1.0 (All-star).
    pub bot_skill: f32,
}

impl PsyonixBotPlayer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the skill of the bot, from 0.0 (Rookie) to 1.0 (All-star).
    pub fn bot_skill(mut self, bot_skill: f32) -> Self {
        self.bot_skill = bot_skill;
        self
    }

    pub(crate) fn build<'fb>(
        &self,
        builder: &mut FlatBufferBuilder<'fb>,
    ) -> WIPOffset<flat::PsyonixBotPlayer<'fb>> {
        let args = flat::PsyonixBotPlayerArgs {
            botSkill: self.bot_skill,
        };
        flat::PsyonixBotPlayer::create(builder, &args)
    }
}

#[derive(Clone)]
pub enum PlayerClass {
    /// A bot controlled by the RLBot framework
    RLBotPlayer,
    /// A normal human player
    HumanPlayer,
    /// A psyonix bot, e.g. All Star bot
    PsyonixBotPlayer(PsyonixBotPlayer),
    /// A player that Rocket League treats as human, e.g. has a dedicated camera
    /// and can do training mode, but is actually controlled by a bot.
    PartyMemberBotPlayer,
}

impl PlayerClass {
    /// A psyonix bot, e.g. All Star bot
    pub fn psyonix_bot(bot_skill: f32) -> Self {
        PlayerClass::PsyonixBotPlayer(PsyonixBotPlayer { bot_skill })
    }

    pub(crate) fn build(
        &self,
        builder: &mut FlatBufferBuilder<'_>,
    ) -> (flat::PlayerClass, WIPOffset<UnionWIPOffset>) {
        match self {
            PlayerClass::RLBotPlayer => (
                flat::PlayerClass::RLBotPlayer,
                flat::RLBotPlayer::create(builder, &flat::RLBotPlayerArgs {}).as_union_value(),
            ),
            PlayerClass::HumanPlayer => (
                flat::PlayerClass::HumanPlayer,
                flat::HumanPlayer::create(builder, &flat::HumanPlayerArgs {}).as_union_value(),
            ),
            PlayerClass::PsyonixBotPlayer(p) => (
                flat::PlayerClass::PsyonixBotPlayer,
                p.build(builder).as_union_value(),
            ),
            PlayerClass::PartyMemberBotPlayer => (
                flat::PlayerClass::PartyMemberBotPlayer,
                flat::PartyMemberBotPlayer::create(builder, &flat::PartyMemberBotPlayerArgs {})
                    .as_union_value(),
            ),
        }
    }
}

/// The car type, color, and other aspects of the player's appearance.
/// See https://github.com/RLBot/RLBot/wiki/Bot-Customization
#[derive(Clone, Default)]
pub struct PlayerLoadout {
    team_color_id: i32,
    custom_color_id: i32,
    car_id: i32,
    decal_id: i32,
    wheels_id: i32,
    boost_id: i32,
    antenna_id: i32,
    hat_id: i32,
    paint_finish_id: i32,
    custom_finish_id: i32,
    engine_audio_id: i32,
    trails_id: i32,
    goal_explosion_id: i32,
    loadout_paint: LoadoutPaint,
}

impl PlayerLoadout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team_color_id(mut self, team_color_id: i32) -> Self {
        self.team_color_id = team_color_id;
        self
    }

    pub fn custom_color_id(mut self, custom_color_id: i32) -> Self {
        self.custom_color_id = custom_color_id;
        self
    }

    pub fn car_id(mut self, car_id: i32) -> Self {
        self.car_id = car_id;
        self
    }

    pub fn decal_id(mut self, decal_id: i32) -> Self {
        self.decal_id = decal_id;
        self
    }

    pub fn wheels_id(mut self, wheels_id: i32) -> Self {
        self.wheels_id = wheels_id;
        self
    }

    pub fn boost_id(mut self, boost_id: i32) -> Self {
        self.boost_id = boost_id;
        self
    }

    pub fn antenna_id(mut self, antenna_id: i32) -> Self {
        self.antenna_id = antenna_id;
        self
    }

    pub fn hat_id(mut self, hat_id: i32) -> Self {
        self.hat_id = hat_id;
        self
    }

    pub fn paint_finish_id(mut self, paint_finish_id: i32) -> Self {
        self.paint_finish_id = paint_finish_id;
        self
    }

    pub fn custom_finish_id(mut self, custom_finish_id: i32) -> Self {
        self.custom_finish_id = custom_finish_id;
        self
    }

    pub fn engine_audio_id(mut self, engine_audio_id: i32) -> Self {
        self.engine_audio_id = engine_audio_id;
        self
    }

    pub fn trails_id(mut self, trails_id: i32) -> Self {
        self.trails_id = trails_id;
        self
    }

    pub fn goal_explosion_id(mut self, goal_explosion_id: i32) -> Self {
        self.goal_explosion_id = goal_explosion_id;
        self
    }

    pub fn loadout_paint(mut self, loadout_paint: LoadoutPaint) -> Self {
        self.loadout_paint = loadout_paint;
        self
    }

    pub(crate) fn build<'fb>(
        &self,
        builder: &mut FlatBufferBuilder<'fb>,
    ) -> WIPOffset<flat::PlayerLoadout<'fb>> {
        let args = flat::PlayerLoadoutArgs {
            teamColorId: self.team_color_id,
            customColorId: self.custom_color_id,
            carId: self.car_id,
            decalId: self.decal_id,
            wheelsId: self.wheels_id,
            boostId: self.boost_id,
            antennaId: self.antenna_id,
            hatId: self.hat_id,
            paintFinishId: self.paint_finish_id,
            customFinishId: self.custom_finish_id,
            engineAudioId: self.engine_audio_id,
            trailsId: self.trails_id,
            goalExplosionId: self.goal_explosion_id,
            loadoutPaint: Some(self.loadout_paint.build(builder)),
        };
        flat::PlayerLoadout::create(builder, &args)
    }
}

/// Specification for 'painted' items. See https://github.com/RLBot/RLBot/wiki/Bot-Customization
#[derive(Clone, Default)]
pub struct LoadoutPaint {
    car_paint_id: i32,
    decal_paint_id: i32,
    wheels_paint_id: i32,
    boost_paint_id: i32,
    antenna_paint_id: i32,
    hat_paint_id: i32,
    trails_paint_id: i32,
    goal_explosion_paint_id: i32,
}

impl LoadoutPaint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn car_paint_id(mut self, car_paint_id: i32) -> Self {
        self.car_paint_id = car_paint_id;
        self
    }

    pub fn decal_paint_id(mut self, decal_paint_id: i32) -> Self {
        self.decal_paint_id = decal_paint_id;
        self
    }

    pub fn wheels_paint_id(mut self, wheels_paint_id: i32) -> Self {
        self.wheels_paint_id = wheels_paint_id;
        self
    }

    pub fn boost_paint_id(mut self, boost_paint_id: i32) -> Self {
        self.boost_paint_id = boost_paint_id;
        self
    }

    pub fn antenna_paint_id(mut self, antenna_paint_id: i32) -> Self {
        self.antenna_paint_id = antenna_paint_id;
        self
    }

    pub fn hat_paint_id(mut self, hat_paint_id: i32) -> Self {
        self.hat_paint_id = hat_paint_id;
        self
    }

    pub fn trails_paint_id(mut self, trails_paint_id: i32) -> Self {
        self.trails_paint_id = trails_paint_id;
        self
    }

    pub fn goal_explosion_paint_id(mut self, goal_explosion_paint_id: i32) -> Self {
        self.goal_explosion_paint_id = goal_explosion_paint_id;
        self
    }

    pub(crate) fn build<'fb>(
        &self,
        builder: &mut FlatBufferBuilder<'fb>,
    ) -> WIPOffset<flat::LoadoutPaint<'fb>> {
        let args = flat::LoadoutPaintArgs {
            carPaintId: self.car_paint_id,
            decalPaintId: self.decal_paint_id,
            wheelsPaintId: self.wheels_paint_id,
            boostPaintId: self.boost_paint_id,
            antennaPaintId: self.antenna_paint_id,
            hatPaintId: self.hat_paint_id,
            trailsPaintId: self.trails_paint_id,
            goalExplosionPaintId: self.goal_explosion_paint_id,
        };
        flat::LoadoutPaint::create(builder, &args)
    }
}

/// Describes one of the players in a match.
#[derive(Clone)]
pub struct PlayerConfiguration<'a> {
    pub variety: PlayerClass,
    pub name: &'a str,
    pub team: i32,
    pub loadout: PlayerLoadout,
    _non_exhaustive: (),
}

impl<'a> PlayerConfiguration<'a> {
    pub fn new(variety: PlayerClass, name: &'a str, team: i32) -> Self {
        Self {
            variety,
            name,
            team,
            loadout: PlayerLoadout::default(),
            _non_exhaustive: (),
        }
    }

    pub fn variety(mut self, variety: PlayerClass) -> Self {
        self.variety = variety;
        self
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn team(mut self, team: i32) -> Self {
        self.team = team;
        self
    }

    pub fn loadout(mut self, loadout: PlayerLoadout) -> Self {
        self.loadout = loadout;
        self
    }

    pub(crate) fn serialize<'fb>(
        &self,
        builder: &mut FlatBufferBuilder<'fb>,
    ) -> WIPOffset<flat::PlayerConfiguration<'fb>> {
        let (variety_type, variety) = self.variety.build(builder);
        let args = flat::PlayerConfigurationArgs {
            variety_type,
            variety: Some(variety),
            name: Some(builder.create_string(self.name)),
            team: self.team,
            loadout: Some(self.loadout.build(builder)),
        };
        flat::PlayerConfiguration::create(builder, &args)
    }
}

#[derive(Clone)]
pub struct MutatorSettings {
    pub match_length: MatchLength,
    pub max_score: MaxScore,
    pub overtime_option: OvertimeOption,
    pub series_length_option: SeriesLengthOption,
    pub game_speed_option: GameSpeedOption,
    pub ball_max_speed_option: BallMaxSpeedOption,
    pub ball_type_option: BallTypeOption,
    pub ball_weight_option: BallWeightOption,
    pub ball_size_option: BallSizeOption,
    pub ball_bounciness_option: BallBouncinessOption,
    pub boost_option: BoostOption,
    pub rumble_option: RumbleOption,
    pub boost_strength_option: BoostStrengthOption,
    pub gravity_option: GravityOption,
    pub demolish_option: DemolishOption,
    pub respawn_time_option: RespawnTimeOption,
    _non_exhaustive: (),
}

impl Default for MutatorSettings {
    fn default() -> Self {
        Self {
            match_length: MatchLength::Five_Minutes,
            max_score: MaxScore::Unlimited,
            overtime_option: OvertimeOption::Unlimited,
            series_length_option: SeriesLengthOption::Unlimited,
            game_speed_option: GameSpeedOption::Default,
            ball_max_speed_option: BallMaxSpeedOption::Default,
            ball_type_option: BallTypeOption::Default,
            ball_weight_option: BallWeightOption::Default,
            ball_size_option: BallSizeOption::Default,
            ball_bounciness_option: BallBouncinessOption::Default,
            boost_option: BoostOption::Normal_Boost,
            rumble_option: RumbleOption::None,
            boost_strength_option: BoostStrengthOption::One,
            gravity_option: GravityOption::Default,
            demolish_option: DemolishOption::Default,
            respawn_time_option: RespawnTimeOption::Three_Seconds,
            _non_exhaustive: (),
        }
    }
}

impl MutatorSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn match_length(mut self, match_length: MatchLength) -> Self {
        self.match_length = match_length;
        self
    }

    pub fn max_score(mut self, max_score: MaxScore) -> Self {
        self.max_score = max_score;
        self
    }

    pub fn overtime_option(mut self, overtime_option: OvertimeOption) -> Self {
        self.overtime_option = overtime_option;
        self
    }

    pub fn series_length_option(mut self, series_length_option: SeriesLengthOption) -> Self {
        self.series_length_option = series_length_option;
        self
    }

    pub fn game_speed_option(mut self, game_speed_option: GameSpeedOption) -> Self {
        self.game_speed_option = game_speed_option;
        self
    }

    pub fn ball_max_speed_option(mut self, ball_max_speed_option: BallMaxSpeedOption) -> Self {
        self.ball_max_speed_option = ball_max_speed_option;
        self
    }

    pub fn ball_type_option(mut self, ball_type_option: BallTypeOption) -> Self {
        self.ball_type_option = ball_type_option;
        self
    }

    pub fn ball_weight_option(mut self, ball_weight_option: BallWeightOption) -> Self {
        self.ball_weight_option = ball_weight_option;
        self
    }

    pub fn ball_size_option(mut self, ball_size_option: BallSizeOption) -> Self {
        self.ball_size_option = ball_size_option;
        self
    }

    pub fn ball_bounciness_option(mut self, ball_bounciness_option: BallBouncinessOption) -> Self {
        self.ball_bounciness_option = ball_bounciness_option;
        self
    }

    pub fn boost_option(mut self, boost_option: BoostOption) -> Self {
        self.boost_option = boost_option;
        self
    }

    pub fn rumble_option(mut self, rumble_option: RumbleOption) -> Self {
        self.rumble_option = rumble_option;
        self
    }

    pub fn boost_strength_option(mut self, boost_strength_option: BoostStrengthOption) -> Self {
        self.boost_strength_option = boost_strength_option;
        self
    }

    pub fn gravity_option(mut self, gravity_option: GravityOption) -> Self {
        self.gravity_option = gravity_option;
        self
    }

    pub fn demolish_option(mut self, demolish_option: DemolishOption) -> Self {
        self.demolish_option = demolish_option;
        self
    }

    pub fn respawn_time_option(mut self, respawn_time_option: RespawnTimeOption) -> Self {
        self.respawn_time_option = respawn_time_option;
        self
    }

    pub(crate) fn build<'fb>(
        &self,
        builder: &mut FlatBufferBuilder<'fb>,
    ) -> WIPOffset<flat::MutatorSettings<'fb>> {
        let args = flat::MutatorSettingsArgs {
            matchLength: self.match_length,
            maxScore: self.max_score,
            overtimeOption: self.overtime_option,
            seriesLengthOption: self.series_length_option,
            gameSpeedOption: self.game_speed_option,
            ballMaxSpeedOption: self.ball_max_speed_option,
            ballTypeOption: self.ball_type_option,
            ballWeightOption: self.ball_weight_option,
            ballSizeOption: self.ball_size_option,
            ballBouncinessOption: self.ball_bounciness_option,
            boostOption: self.boost_option,
            rumbleOption: self.rumble_option,
            boostStrengthOption: self.boost_strength_option,
            gravityOption: self.gravity_option,
            demolishOption: self.demolish_option,
            respawnTimeOption: self.respawn_time_option,
        };
        flat::MutatorSettings::create(builder, &args)
    }
}

/// Options for starting a match.
///
/// Pass this to [`RLBot::start_match`](crate::RLBot::start_match)` to make the
/// magic happen.
#[derive(Clone)]
pub struct MatchSettings<'a> {
    pub player_configurations: Vec<PlayerConfiguration<'a>>,
    pub game_mode: GameMode,
    pub game_map: GameMap,
    pub skip_replays: bool,
    pub instant_start: bool,
    pub mutator_settings: MutatorSettings,
    _non_exhaustive: (),
}

impl<'a> Default for MatchSettings<'a> {
    fn default() -> Self {
        Self {
            player_configurations: Vec::new(),
            game_mode: GameMode::Soccer,
            game_map: GameMap::DFHStadium,
            skip_replays: false,
            instant_start: false,
            mutator_settings: Default::default(),
            _non_exhaustive: (),
        }
    }
}

impl<'a> MatchSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a `MatchSettings` for a 1v1 game with Team Blue as an
    /// RLBot-controlled bot, and Team Orange as a Psyonix all-star bot.
    pub fn rlbot_vs_allstar(rlbot_name: &'a str, allstar_name: &'a str) -> Self {
        Self::new().player_configurations(vec![
            PlayerConfiguration::new(PlayerClass::RLBotPlayer, rlbot_name, 0),
            PlayerConfiguration::new(PlayerClass::psyonix_bot(1.0), allstar_name, 1),
        ])
    }

    /// Create a `MatchSettings` for a 1v1 game with two Psyonix all-star bots.
    pub fn allstar_vs_allstar(blue_name: &'a str, orange_name: &'a str) -> Self {
        Self::new().player_configurations(vec![
            PlayerConfiguration::new(PlayerClass::psyonix_bot(1.0), blue_name, 0),
            PlayerConfiguration::new(PlayerClass::psyonix_bot(1.0), orange_name, 1),
        ])
    }

    pub fn player_configurations(
        mut self,
        player_configurations: impl IntoIterator<Item = PlayerConfiguration<'a>>,
    ) -> Self {
        self.player_configurations = player_configurations.into_iter().collect();
        self
    }

    pub fn game_mode(mut self, game_mode: GameMode) -> Self {
        self.game_mode = game_mode;
        self
    }

    pub fn game_map(mut self, game_map: GameMap) -> Self {
        self.game_map = game_map;
        self
    }

    pub fn skip_replays(mut self, skip_replays: bool) -> Self {
        self.skip_replays = skip_replays;
        self
    }

    pub fn instant_start(mut self, instant_start: bool) -> Self {
        self.instant_start = instant_start;
        self
    }

    pub fn mutator_settings(mut self, mutator_settings: MutatorSettings) -> Self {
        self.mutator_settings = mutator_settings;
        self
    }

    pub(crate) fn build(&self) -> FlatBufferBuilder<'_> {
        let mut builder = FlatBufferBuilder::new_with_capacity(1024);

        let player_configurations = self
            .player_configurations
            .iter()
            .map(|x| x.serialize(&mut builder))
            .collect::<Vec<_>>();
        let args = flat::MatchSettingsArgs {
            playerConfigurations: Some(builder.create_vector(&player_configurations)),
            gameMode: self.game_mode,
            gameMap: self.game_map,
            skipReplays: false,
            instantStart: false,
            mutatorSettings: Some(self.mutator_settings.build(&mut builder)),
        };
        let root = flat::MatchSettings::create(&mut builder, &args);

        builder.finish(root, None);
        builder
    }
}
