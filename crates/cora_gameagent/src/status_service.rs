use tonic::{Request, Response, Status};

use status_service::status_service_rpc_server::StatusServiceRpc;
use status_service::PingRequest;
use status_service::PingResponse;

pub mod status_service {
    tonic::include_proto!("statusservice");
}

#[derive(Debug, Default)]
pub struct StatusService {}

#[tonic::async_trait]
impl StatusServiceRpc for StatusService {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let response = PingResponse {};
        Ok(Response::new(response))
    }
}
