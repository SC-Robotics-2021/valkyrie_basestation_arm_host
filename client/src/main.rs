use anyhow;
use linux_embedded_hal::I2cdev;
use jrk_g2_rs::{JrkG2I2c, JrkG2};
fn main() -> anyhow::Result<()>{

    println!("Opening I2C bus...");
    let i2c = I2cdev::new("/dev/i2c-1")?;

    let mut jrk_handle = JrkG2I2c::new(i2c);

    jrk_handle.set_target(255)?;
    Ok(())
}