mod pi;
mod temp;
mod monitor;

use anyhow::Result;
use monitor::Monitor;

fn main() -> Result<()> {
    Monitor::builder(
        env!("RADIATOR_PIN").parse::<u8>()?, 
        env!("RADIATOR_DELAY").parse::<u64>()?
    )?.run()
}
