use crate::{
    error::RLBotError,
    game::{build_update_player_input, ControllerState},
    interface::RLBotInterface,
    match_settings::MatchSettings,
    packeteer::Packeteer,
    physicist::Physicist,
    render::RenderGroup,
    state,
};
use std::{cell::Cell, error::Error, marker::PhantomData};

/// The low-level interface to RLBot. All RLBot calls that are available can be
/// made through this struct.
pub struct RLBot {
    interface: RLBotInterface,
    /// I strongly doubt the RLBot DLL is thread-safe, so let's enforce that
    /// restriction.
    ///
    /// This is the abstruse equivalent of `impl !Sync` in nightly Rust.
    not_sync: PhantomData<Cell<()>>,
}

impl RLBot {
    pub(crate) fn new(interface: RLBotInterface) -> Self {
        Self {
            interface,
            not_sync: PhantomData,
        }
    }

    /// Gives direct access to the FFI methods in `RLBotInterface.dll`.
    pub fn interface(&self) -> &RLBotInterface {
        &self.interface
    }

    /// Returns a [`Packeteer`] object, for conveniently accessing game state
    /// as it occurs.
    pub fn packeteer(&self) -> Packeteer<'_> {
        Packeteer::new(self)
    }

    /// Returns a [`Physicist`] object, for conveniently accessing physics
    /// ticks as they occur.
    pub fn physicist(&self) -> Physicist<'_> {
        Physicist::new(self)
    }

    /// Sends player input to RLBot.
    pub fn update_player_input(
        &self,
        player_index: i32,
        controller_state: &ControllerState,
    ) -> Result<(), RLBotError> {
        let built = build_update_player_input(player_index, controller_state);
        self.interface
            .update_player_input_flatbuffer(built.finished_data())
    }

    /// Sets the game state.
    pub fn set_game_state(
        &self,
        desired_game_state: &state::DesiredGameState,
    ) -> Result<(), RLBotError> {
        let buffer = desired_game_state.serialize();
        self.interface.set_game_state(buffer.finished_data())
    }

    /// Tells RLBot to start a match.
    pub fn start_match(&self, match_settings: &MatchSettings<'_>) -> Result<(), Box<dyn Error>> {
        let buffer = match_settings.build();
        self.interface
            .start_match_flatbuffer(buffer.finished_data())?;
        Ok(())
    }

    /// Spin-waits until a match is active.
    ///
    /// Call `start_match` before calling this method.
    pub fn wait_for_match_start(&self) -> Result<(), Box<dyn Error>> {
        let mut packets = self.packeteer();
        let mut count = 0;

        // Sometimes we get a few stray ticks from a previous game while the next game
        // is loading. Wait for RoundActive to stabilize before trusting it.
        while count < 5 {
            let packet = packets.next_flatbuffer()?;
            let is_round_active = packet
                .gameInfo()
                .map(|gi| gi.isRoundActive())
                .unwrap_or_default();
            if is_round_active {
                count += 1;
            } else {
                count = 0;
            }
        }
        Ok(())
    }

    /// Begin drawing to the screen.
    ///
    /// The ID identifies a group. Multiple groups can exist and be updated
    /// independently. Drawings will remain on screen until they are replaced
    /// by a group with the same ID.
    ///
    /// A group can be cleared from the screen by rendering an empty group.
    ///
    /// See [`RenderGroup`] for more info.
    pub fn begin_render_group(&self, id: i32) -> RenderGroup<'_> {
        RenderGroup::new(self, id)
    }
}

#[cfg(test)]
mod tests {
    use crate::rlbot::RLBot;
    use std::{error::Error, mem};

    #[test]
    #[ignore(note = "compile-only test")]
    fn game_data_is_send() -> Result<(), Box<dyn Error>> {
        fn assert_send<T: Send + 'static>(_: T) {}

        let rlbot: RLBot = unsafe { mem::uninitialized() };
        assert_send(rlbot.physicist().next_flat()?);
        assert_send(rlbot.packeteer().next()?);
        assert_send(rlbot.packeteer().next_flatbuffer()?);
        Ok(())
    }
}
