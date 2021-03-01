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
    let mut cnc_client = CommandAndControlServiceClient::connect("http://xavier.rover.theunknown.dev").await?;


    let pose = kinematic_client.get_arm_state(Empty {}).await?.into_inner();

    println!("response.lower_axis := {:?}", pose.lower_axis);
    println!("response.upper_axis := {:?}", pose.upper_axis);
    println!("response.rotation := {:?}", pose.rotation);
    println!("response.driving_arm := {:?}", pose.driving_arm);
    println!("response.driving_gripper := {:?}", pose.driving_gripper);

    cnc_client.set_arm(pose).await?;



    Ok(())
}
