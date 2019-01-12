use crate::{ffi, flat, rlbot::RLBot};
use std::{
    error::Error,
    mem,
    time::{Duration, Instant},
};

/// An iterator-like object that yields physics ticks from the game as they
/// occur.
pub struct Physicist<'a> {
    rlbot: &'a RLBot,
    ratelimiter: ratelimit::Limiter,
    prev_ball_frame: i32,
}

impl<'a> Physicist<'a> {
    pub(crate) fn new(rlbot: &RLBot) -> Physicist<'_> {
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
    /// This function returns an error if ten seconds pass without a new tick
    /// being received. The assumption is that the game froze or crashed, and
    /// waiting longer will not help.
    pub fn next(&mut self) -> Result<ffi::RigidBodyTick, Box<dyn Error>> {
        self.spin(|this| Ok(this.try_next()?))
    }

    /// Polls for a new physics tick.
    ///
    /// If there is a tick that is newer than the previous tick, it is
    /// returned. Otherwise, `None` is returned.
    pub fn try_next(&mut self) -> Result<Option<ffi::RigidBodyTick>, Box<dyn Error>> {
        let mut result = unsafe { mem::uninitialized() };
        self.rlbot.update_rigid_body_tick(&mut result)?;
        if result.Ball.State.Frame != self.prev_ball_frame {
            self.prev_ball_frame = result.Ball.State.Frame;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Block until the next physics tick occurs, and then return it.
    ///
    /// # Errors
    ///
    /// This function returns an error if ten seconds pass without a new tick
    /// being received. The assumption is that the game froze or crashed, and
    /// waiting longer will not help.
    pub fn next_flat<'fb>(&mut self) -> Result<flat::RigidBodyTick<'fb>, Box<dyn Error>> {
        self.spin(|this| Ok(this.try_next_flat()))
    }

    /// Polls for a new physics tick.
    ///
    /// If there is a tick that is newer than the previous tick, it is
    /// returned. Otherwise, `None` is returned.
    pub fn try_next_flat<'fb>(&mut self) -> Option<flat::RigidBodyTick<'fb>> {
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
