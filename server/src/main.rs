mod server;

use server::KinematicArmServer;
use tonic::{transport::Server, Request, Response, Status};
use rover_tonic::borealis::kinematic_arm_state_servicer_server::KinematicArmStateServicerServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let addr = "[::1]:50051".parse().unwrap();
    let addr = "[::]:50051".parse().unwrap();
    let server = KinematicArmServer::new("/dev/ttyS0")?;
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(KinematicArmStateServicerServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
