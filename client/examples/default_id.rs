use jrk_g2_rs::{JrkG2Serial, JrkG2, VarOffset};
use linux_embedded_hal::Serial;

fn main() -> anyhow::Result<()> {

    let mut jrk = JrkG2Serial::new(Serial::open("/dev/ttyACM3".as_ref())?);

    println!("fetching target...");
    let target = jrk.read(VarOffset::Target).expect("failed to read target");
    println!("fetched target := {:?}", target);

    println!("setting target ....");

    jrk.set_target(1700).expect("failed to set target.");
    println!("fetching target...");
    let target = jrk.read(VarOffset::Target).expect("failed to read target");
    println!("fetched target := {:?}", target);
    Ok(())
}