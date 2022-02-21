use tokio;

mod driver;

use rover_tonic::borealis::kinematic_arm_state_servicer_client::KinematicArmStateServicerClient;
use rover_tonic::borealis::command_and_control_service_client::CommandAndControlServiceClient;
use rover_tonic::borealis::Empty;
use tonic::transport::Channel;
use std::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut kinematic_client = KinematicArmStateServicerClient::connect("http://localhost:50051").await?;
    let mut cnc_client = CommandAndControlServiceClient::connect("http://telecom.rover.theunknown.dev:50051").await?;

    loop {
        let mut pose = kinematic_client.get_arm_state(Empty {}).await?.into_inner();
        pose.upper_axis *= -1.0f32;
        pose.rotation *= -1.0f32;
        print!("response.lower_axis := {:?}\t", pose.lower_axis);
        print!("response.upper_axis := {:?}\t", pose.upper_axis);
        print!("response.rotation := {:?}\t", pose.rotation);
        print!("response.driving_arm := {:?}\t", pose.driving_arm);
        println!("response.driving_gripper := {:?}", pose.driving_gripper);

        cnc_client.set_arm(pose).await?;
    }


    Ok(())
}
