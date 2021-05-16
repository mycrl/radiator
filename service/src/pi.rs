use std::sync::Once;
use std::os::raw::{
    c_int,
    c_uint
};

use anyhow::{
    Result,
    anyhow
};

/// ```c
/// #define PI_OUTPUT 1
/// ```
const PI_OUTPUT: c_uint = 1;

/// Only need to call gpioInitialise once, 
/// once is used to run a one-time global initialization.
static PI_SETUP: Once = Once::new();

#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    /// ```c
    /// int gpioInitialise(void);
    /// ````
    ///
    /// Initialises the library.
    /// Returns the pigpio version number if OK, otherwise PI_INIT_FAILED.
    ///
    /// # Example
    ///
    /// ```c
    /// if (gpioInitialise() < 0) {
    ///     // pigpio initialisation failed.
    /// } else {
    ///     // pigpio initialised okay.
    /// }
    /// ```
    fn gpioInitialise() -> c_int;
    /// ```c
    /// int gpioSetMode(unsigned pin, unsigned mode);
    /// ````
    /// 
    /// Sets the GPIO mode, typically input or output.
    ///
    /// ```no_run
    /// gpio: 0-53
    /// mode: 0-7
    /// ```
    /// 
    /// Returns 0 if OK, otherwise PI_BAD_GPIO or PI_BAD_MODE.
    /// 
    /// # Example
    ///
    /// ```c
    /// gpioSetMode(17, PI_INPUT);  // Set GPIO17 as input.
    /// gpioSetMode(18, PI_OUTPUT); // Set GPIO18 as output.
    /// gpioSetMode(22,PI_ALT0);    // Set GPIO22 to alternative mode 0.
    /// ```
    fn gpioSetMode(pin: c_uint, mode: c_uint) -> c_int;
    /// ```c
    /// int gpioPWM(unsigned pin, unsigned dutycycle);
    /// ````
    /// 
    /// Starts PWM on the GPIO, dutycycle between 0 (off) and range (fully on). 
    /// Range defaults to 255.
    ///
    /// ```no_run
    /// user_gpio: 0-31
    /// dutycycle: 0-range
    /// ```
    /// 
    /// Returns 0 if OK, otherwise PI_BAD_USER_GPIO or PI_BAD_DUTYCYCLE.
    /// 
    /// # Example
    ///
    /// ```c
    /// gpioSetMode(17, PI_INPUT);  // Set GPIO17 as input.
    /// gpioSetMode(18, PI_OUTPUT); // Set GPIO18 as output.
    /// gpioSetMode(22,PI_ALT0);    // Set GPIO22 to alternative mode 0.
    /// ```
    fn gpioPWM(pin: c_uint, value: c_uint) -> c_int;
}

/// RaspberryPI fan
pub struct Fan {
    pin: u8
}

impl Fan {
    /// Specify PWM pin to create Fan instance.
    /// 
    /// note: Only supports Hardware PWM! Hardware PWM available 
    /// on GPIO12, GPIO13, GPIO18, GPIO19.
    ///
    /// #Example
    ///
    /// ```
    /// Fan::new(12).unwrap();
    /// // panic or ok
    /// ```
    #[rustfmt::skip]
    pub fn new(pin: u8) -> Result<Self> {
        PI_SETUP.call_once(|| {
            if unsafe { gpioInitialise() } < 0 {
                panic!();
            }
        });

        if unsafe { gpioSetMode(pin as c_uint, PI_OUTPUT) } != 0 {
            return Err(anyhow!("gpioSetMode failed!"))
        }

        Ok(Fan {
            pin
        })
    }

    /// Update PWM pin duty-cycle.
    /// 
    /// note: dutycycle between 0 (off) and 255 (fully on).
    ///
    /// #Example
    ///
    /// ```
    /// let fan = Fan::new(12).unwrap();
    /// fan.up(0).unwrap();
    /// // panic or ok
    /// ```
    #[rustfmt::skip]
    pub fn up(&mut self, value: u8) -> Result<()> {
        if unsafe { gpioPWM(self.pin as c_uint, value as c_uint) } != 0 {
            return Err(anyhow!("gpioPWM failed!"))
        }

        Ok(())
    }
}
