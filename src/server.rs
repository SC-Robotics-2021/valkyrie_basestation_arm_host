use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
mod hello;

#[derive(Default)]
pub struct SayServicer {}

#[tonic::async_trait]
impl Say for SayServicer {
// defining return stream
    type bidirectionalStream = mpsc::Receiver<Result<SayResponse, Status>>;
    async fn bidirectional(
        &self,
        request: Request<tonic::Streaming<SayRequest>>,
    ) -> Result<Response<Self::bidirectionalStream>, Status> {
// converting request in stream
        let mut streamer = request.into_inner();
// creating queue
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
// listening on request stream
            while let Some(req) = streamer.message().await.unwrap(){
// sending data as soon it is available
                println!("Server recieved {}", req.grip3);
                tx.send(Ok(SayResponse {
                    base: req.base,
                    lower: req.lower,
                    upper: req.upper,
                    grip1: req.grip1,
                    grip2: req.grip2,
                    grip3: req.grip3,
                }))
                .await
                .unwrap();
            }
        });
// returning stream as receiver
        Ok(Response::new(rx))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let addr = "[::1]:50051".parse().unwrap();
    let addr = "[::]:50051".parse().unwrap();
    let say = SayServicer::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
