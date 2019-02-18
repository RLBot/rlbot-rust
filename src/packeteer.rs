#![allow(clippy::float_cmp)]

use crate::{ffi::LiveDataPacket, rlbot::RLBot, rlbot_generated::rlbot::flat::GameTickPacket};
use std::{
    error::Error,
    mem,
    time::{Duration, Instant},
};

/// An iterator-like object that yields packets from the game as they occur.
pub struct Packeteer<'a> {
    rlbot: &'a RLBot,
    ratelimiter: ratelimit::Limiter,
    prev_game_time: f32,
}

impl<'a> Packeteer<'a> {
    pub(crate) fn new(rlbot: &'a RLBot) -> Self {
        // The goal is never to miss any packets. But if we poll too often, the
        // game crashes, so it's a fine line. With an interval of 3ms we can
        // catch 333 updates per second. That should be plenty.
        let ratelimiter = ratelimit::Builder::new()
            .interval(Duration::from_millis(3))
            .build();

        Self {
            rlbot,
            ratelimiter,
            prev_game_time: 0.0,
        }
    }

    /// Block until we receive the next unique [`LiveDataPacket`], and then
    /// return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if ten seconds pass without a new
    /// packet being received. The assumption is that the game froze or
    /// crashed, and waiting longer will not help.
    #[allow(clippy::should_implement_trait)]
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    #[allow(deprecated)]
    pub fn next(&mut self) -> Result<LiveDataPacket, Box<dyn Error>> {
        self.spin(Self::try_next)
    }

    /// Polls for the next [`LiveDataPacket`].
    ///
    /// If there is a packet that is newer than the previous packet, it is
    /// returned. Otherwise, `None` is returned.
    #[deprecated(
        note = "the struct-based methods are deprecated; use the flatbuffer equivalents instead"
    )]
    #[allow(deprecated)]
    pub fn try_next(&mut self) -> Result<Option<LiveDataPacket>, Box<dyn Error>> {
        let mut packet = unsafe { mem::uninitialized() };
        self.rlbot.interface.update_live_data_packet(&mut packet)?;

        let game_time = packet.GameInfo.TimeSeconds;
        if game_time != self.prev_game_time {
            self.prev_game_time = game_time;
            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }

    /// Block until we receive the next unique [`GameTickPacket`], and then
    /// return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if ten seconds pass without a new
    /// packet being received. The assumption is that the game froze or
    /// crashed, and waiting longer will not help.
    pub fn next_flatbuffer<'fb>(&mut self) -> Result<GameTickPacket<'fb>, Box<dyn Error>> {
        self.spin(|this| Ok(this.try_next_flat()))
    }

    /// Polls for the next [`GameTickPacket`].
    ///
    /// If there is a packet that is newer than the previous packet, it is
    /// returned. Otherwise, `None` is returned.
    pub fn try_next_flat<'fb>(&mut self) -> Option<GameTickPacket<'fb>> {
        if let Some(packet) = self.rlbot.interface.update_live_data_packet_flatbuffer() {
            let game_time = packet.gameInfo().map(|gi| gi.secondsElapsed());
            if let Some(game_time) = game_time {
                if game_time != self.prev_game_time {
                    self.prev_game_time = game_time;
                    return Some(packet);
                }
            }
        }
        None
    }

    /// Keep trying `f` until the timeout elapses.
    fn spin<R>(
        &mut self,
        f: impl Fn(&mut Self) -> Result<Option<R>, Box<dyn Error>>,
    ) -> Result<R, Box<dyn Error>> {
        let start = Instant::now();

        loop {
            self.ratelimiter.wait();

            if let Some(tick) = f(self)? {
                return Ok(tick);
            }

            let elapsed = Instant::now() - start;
            if elapsed > Duration::from_secs(10) {
                return Err(From::from("no physics tick received after ten seconds"));
            }
        }
    }
}
