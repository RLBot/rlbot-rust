use flat;
use ratelimit;
use rlbot::RLBot;
use std::error::Error;
use std::time::{Duration, Instant};

/// An iterator-like object that yields physics ticks from the game as they
/// occur.
pub struct Physicist<'a> {
    rlbot: &'a RLBot,
    ratelimiter: ratelimit::Limiter,
    prev_ball_frame: i32,
}

impl<'a> Physicist<'a> {
    pub(crate) fn new(rlbot: &RLBot) -> Physicist {
        // Physics ticks happen at 120Hz. The goal is never to miss any. But if we poll
        // too often, the game crashes, so space out the checks.
        let ratelimiter = ratelimit::Builder::new()
            .interval(Duration::from_millis(1))
            .build();

        Physicist {
            rlbot,
            ratelimiter,
            prev_ball_frame: 0,
        }
    }

    /// Block until the next physics tick occurs, and then return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if five seconds pass without a new tick
    /// being received. The assumption is that the game froze or crashed, and
    /// waiting longer will not help.
    pub fn next_flat(&mut self) -> Result<flat::RigidBodyTick<'a>, Box<Error>> {
        let started = Instant::now();

        loop {
            self.ratelimiter.wait();

            if let Some(tick) = self.try_next_flat() {
                return Ok(tick);
            }

            if Instant::now() - started > Duration::from_secs(5) {
                return Err(From::from("no physics tick received after five seconds"));
            }
        }
    }

    /// Polls for a new physics tick.
    ///
    /// If there is a tick that is newer than the previous tick, it is
    /// returned. Otherwise, `None` is returned.
    pub fn try_next_flat(&mut self) -> Option<flat::RigidBodyTick<'a>> {
        if let Some(tick) = self.rlbot.update_rigid_body_tick_flatbuffer() {
            let ball = tick.ball();
            match ball.as_ref().and_then(|b| b.state()).map(|s| s.frame()) {
                Some(ball_frame) if ball_frame != self.prev_ball_frame => {
                    self.prev_ball_frame = ball_frame;
                    return Some(tick);
                }
                _ => {}
            }
        }
        None
    }
}
