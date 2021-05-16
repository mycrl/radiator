use anyhow::Result;
use std::time::Duration;
use super::{
    pi::Fan,
    temp::*
};

use std::thread::{
    sleep, 
    spawn
};

/// Temperature monitor.
pub struct Monitor {
    poll_delay: Duration,
    fan: Fan
}

impl Monitor {
    /// Created monitor.
    ///
    /// Specify the fan speed control pin and loop cycle(secs).
    ///
    /// #Example
    ///
    /// ```
    /// Monitor::new(12, 5000).unwrap();
    /// // panic or ok
    /// ```
    #[rustfmt::skip]
    pub fn builder(pin: u8, delay: u64) -> Result<Self> {
        Ok(Self {
            poll_delay: Duration::from_secs(delay),
            fan: Fan::new(pin)?
        })
    }

    /// Monitor execution.
    ///
    /// Get the soc temperature and submit the duty cycle to the PWM pin.
    ///
    /// #Example
    ///
    /// ```
    /// let mut monitor = Monitor::new(12, 5000).unwrap();
    /// loop { monitor.poll().unwrap() }
    /// ```
    #[rustfmt::skip]
    pub fn poll(&mut self) -> Result<()> {
        self.fan.up(get_pwm(get_temp()?))?;
        sleep(self.poll_delay);
        Ok(())
    }
    
    /// Running monitor in independent thread.
    /// 
    /// #Example
    ///
    /// ```
    /// Monitor::new(12, 5000)
    ///     .unwrap()
    ///     .run()
    ///     .unwrap();
    /// ```
    #[rustfmt::skip]
    #[allow(unreachable_code)]
    pub fn run(self) -> Result<()> {
        spawn(move || {
            let mut this = self;
            loop { this.poll()? }
            Ok::<(), anyhow::Error>(())
        })
        .join()
        .unwrap()
    }
}
