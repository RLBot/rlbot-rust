use ffi;

impl ffi::LiveDataPacket {
    /// Yields the [`PlayerInfo`](ffi::PlayerInfo) for each player in the match.
    pub fn cars(&self) -> impl Iterator<Item = &ffi::PlayerInfo> {
        self.GameCars.iter().take(self.NumCars as usize)
    }

    /// Get the scores for the blue and orange teams, in that order.
    ///
    /// RLBot doesn't seem to return this info(?) so instead we compute it
    /// manually (and inaccurately) by adding up the goals for each individual
    /// player.
    pub fn match_score(&self) -> [i32; 2] {
        let mut result = [0, 0];
        for car in self.cars() {
            result[car.Team as usize] += car.Score.Goals;
        }
        result
    }
}

impl ffi::MatchSettings {
    /// Create a simple 1v1 match with sensible defaults and the given player
    /// names. Team Blue will be RLBot-controlled, and Team Orange will be a
    /// Psyonix all-star bot.
    pub fn simple_1v1(hero: &str, villain: &str) -> Self {
        let mut result = ffi::MatchSettings {
            NumPlayers: 2,
            ..Default::default()
        };

        result.PlayerConfiguration[0].Bot = true;
        result.PlayerConfiguration[0].RLBotControlled = true;
        result.PlayerConfiguration[0].set_name(hero);

        result.PlayerConfiguration[1].Bot = true;
        result.PlayerConfiguration[1].BotSkill = 1.0;
        result.PlayerConfiguration[1].set_name(villain);
        result.PlayerConfiguration[1].Team = 1;

        result
    }
}

impl ffi::PlayerConfiguration {
    /// Populate the `Name` field from a string.
    pub fn set_name(&mut self, name: &str) {
        for (i, cp) in name.encode_utf16().enumerate() {
            self.Name[i] = cp;
        }
    }
}
