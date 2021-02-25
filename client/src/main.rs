use tokio;

mod driver;

use rover_tonic::borealis::kinematic_arm_state_servicer_client::KinematicArmStateServicerClient;
use rover_tonic::borealis::Empty;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = KinematicArmStateServicerClient::connect("http://localhost:50051").await?;

    let pose = client.get_arm_state(Empty {}).await?.into_inner();

    println!("response.lower_axis := {:?}", pose.lower_axis);
    println!("response.upper_axis := {:?}", pose.upper_axis);
    println!("response.rotation := {:?}", pose.rotation);
    println!("response.driving_arm := {:?}", pose.driving_arm);
    println!("response.driving_gripper := {:?}", pose.driving_gripper);


    Ok(())
}

