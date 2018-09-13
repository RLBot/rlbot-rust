use ffi::LiveDataPacket;
use ratelimit;
use rlbot::RLBot;
use rlbot_generated::rlbot::flat::GameTickPacket;
use std::error::Error;
use std::mem;
use std::time::{Duration, Instant};

/// An iterator-like object that yields
/// [`LiveDataPacket`](::ffi::LiveDataPacket)s or [`GameTickPacket`](::flat::
/// GameTickPacket)s from the game as they occur.
pub struct Packeteer<'a> {
    rlbot: &'a RLBot,
    ratelimiter: ratelimit::Limiter,
    prev_game_time: f32,
}

impl<'a> Packeteer<'a> {
    pub(crate) fn new(rlbot: &RLBot) -> Packeteer {
        // The goal is never to miss any packets. But if we poll too often, the
        // game crashes, so it's a fine line. With an interval of 3ms we can
        // catch 333 updates per second. That should be plenty.
        let ratelimiter = ratelimit::Builder::new()
            .interval(Duration::from_millis(3))
            .build();

        Packeteer {
            rlbot,
            ratelimiter,
            prev_game_time: 0.0,
        }
    }

    /// Block until we receive the next unique
    /// [`LiveDataPacket`](::ffi::LiveDataPacket), and then return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if five seconds pass without a new
    /// packet being received. The assumption is that the game froze or
    /// crashed, and waiting longer will not help.
    pub fn next(&mut self) -> Result<LiveDataPacket, Box<Error>> {
        let started = Instant::now();
        let mut packet = unsafe { mem::uninitialized() };

        loop {
            self.ratelimiter.wait();

            self.rlbot.update_live_data_packet(&mut packet)?;

            // Wait until another "tick" has happened so we don't return duplicate data.
            let game_time = packet.GameInfo.TimeSeconds;
            if game_time != self.prev_game_time {
                self.prev_game_time = game_time;
                break;
            }

            if Instant::now() - started > Duration::from_secs(5) {
                return Err(From::from("no packet received after five seconds"));
            }
        }

        Ok(packet)
    }

    /// Block until we receive the next unique
    /// [`GameTickPacket`](::flat::GameTickPacket), and then return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if ten seconds pass without a new
    /// packet being received. The assumption is that the game froze or
    /// crashed, and waiting longer will not help.
    pub fn next_flatbuffer(&mut self) -> Result<GameTickPacket, Box<Error>> {
        let started = Instant::now();

        loop {
            self.ratelimiter.wait();

            if let Some(packet) = self.rlbot.update_live_data_packet_flatbuffer() {
                // Wait until another "tick" has happened so we don't return duplicate data.
                let game_time = packet
                    .gameInfo()
                    .ok_or("Missing gameInfo")?
                    .secondsElapsed();
                if game_time != self.prev_game_time {
                    self.prev_game_time = game_time;
                    return Ok(packet);
                }
            }

            if Instant::now() - started > Duration::from_secs(10) {
                return Err(From::from("no packet received after ten seconds"));
            }
        }
    }
}
