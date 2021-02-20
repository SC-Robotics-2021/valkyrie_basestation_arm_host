use futures::stream::iter;
use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = SayClient::new(channel);
// creating a client
    let request = tonic::Request::new(iter(vec![
        SayRequest {
            base: 1,
            lower: 2,
            upper: 3,
            grip1: 4,
            grip2: 5,
            grip3: 6
        },
        SayRequest {
            base: 1,
            lower: 2,
            upper: 3,
            grip1: 4,
            grip2: 5,
            grip3: 7
        },
        SayRequest {
            base: 1,
            lower: 2,
            upper: 3,
            grip1: 4,
            grip2: 5,
            grip3: 8
        },
    ]));
// calling rpc
    let mut response = client.bidirectional(request).await?.into_inner();
// listening on the response stream
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}
