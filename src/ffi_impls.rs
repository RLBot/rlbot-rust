use crate::{ffi, ffi::ByteBuffer};
use std::ptr;

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
    #[doc(hidden)]
    #[deprecated(note = "this method has been renamed to `rlbot_vs_allstar`")]
    pub fn simple_1v1(rlbot_name: &str, allstar_name: &str) -> Self {
        Self::rlbot_vs_allstar(rlbot_name, allstar_name)
    }

    /// Create a `MatchSettings` for a 1v1 game with Team Blue as an
    /// RLBot-controlled bot, and Team Orange as a Psyonix all-star bot.
    pub fn rlbot_vs_allstar(rlbot_name: &str, allstar_name: &str) -> Self {
        let mut result = ffi::MatchSettings {
            NumPlayers: 2,
            ..Default::default()
        };

        result.PlayerConfiguration[0].Bot = true;
        result.PlayerConfiguration[0].RLBotControlled = true;
        result.PlayerConfiguration[0].set_name(rlbot_name);

        result.PlayerConfiguration[1].Bot = true;
        result.PlayerConfiguration[1].BotSkill = 1.0;
        result.PlayerConfiguration[1].set_name(allstar_name);
        result.PlayerConfiguration[1].Team = 1;

        result
    }

    /// Create a `MatchSettings` for a 1v1 game with two Psyonix all-star bots.
    pub fn allstar_vs_allstar(blue_name: &str, orange_name: &str) -> Self {
        let mut result = ffi::MatchSettings {
            NumPlayers: 2,
            ..Default::default()
        };

        result.PlayerConfiguration[0].Bot = true;
        result.PlayerConfiguration[0].BotSkill = 1.0;
        result.PlayerConfiguration[0].set_name(blue_name);

        result.PlayerConfiguration[1].Bot = true;
        result.PlayerConfiguration[1].BotSkill = 1.0;
        result.PlayerConfiguration[1].set_name(orange_name);
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

impl From<ByteBuffer> for Option<Vec<u8>> {
    fn from(byte_buffer: ByteBuffer) -> Self {
        let len = byte_buffer.size as usize;
        if len == 0 || byte_buffer.ptr.is_null() {
            return None;
        }

        let mut buf = Vec::with_capacity(len);
        unsafe {
            ptr::copy_nonoverlapping(byte_buffer.ptr as *const u8, buf.as_mut_ptr(), len);
            buf.set_len(len);
        }

        Some(buf)
    }
}
