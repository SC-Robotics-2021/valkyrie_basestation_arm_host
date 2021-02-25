mod server;

use rover_tonic::borealis::kinematic_arm_state_servicer_server::KinematicArmStateServicerServer;
use server::KinematicArmServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::]:50051".parse()?;
    let server = KinematicArmServer::new("/dev/ttyUSB0")?;
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(KinematicArmStateServicerServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
