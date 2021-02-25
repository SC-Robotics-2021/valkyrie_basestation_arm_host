use std::sync::Arc;
use tokio::sync::Mutex;
use jrk_g2_rs::{JrkG2I2c, JrkG2Serial};
use linux_embedded_hal::{I2cdev, Serial};

pub(crate) struct I2cDriver {
    pub(crate) jrk: Arc<Mutex<JrkG2I2c<I2cdev>>>
}

impl I2cDriver {
    pub fn new(buss: I2cdev) -> anyhow::Result<Self> {
        println!("Opening I2C bus...");
        Ok(
            Self {
                jrk: Arc::new(Mutex::new(JrkG2I2c::new(buss)))
            }
        )
    }
}

pub(crate) struct SerialDriver {
    pub(crate) jrk: Arc<std::sync::Mutex<JrkG2Serial<Serial>>>
}

impl SerialDriver{
    pub fn new(buss: Serial) -> anyhow::Result<Self> {
        println!("Opening I2C bus...");
        Ok(
            Self {
                jrk: Arc::new(std::sync::Mutex::new(JrkG2Serial::new(buss)))
            }
        )
    }
}