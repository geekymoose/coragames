mod status_service;

use tonic::transport::Server;
use status_service::StatusService;
use status_service::status_service::status_service_rpc_server::StatusServiceRpcServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cora GameAgent stars...");

    let addr = "[::1]:50051".parse()?;
    let status_service = StatusService::default();

    Server::builder()
        .add_service(StatusServiceRpcServer::new(status_service))
        .serve(addr)
        .await?;

    println!("Cora GameAgent stops...");
    Ok(())
}