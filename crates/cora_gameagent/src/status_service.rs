use tonic::{transport::Server, Request, Response, Status};


pub mod status_service {
    tonic::include_proto!("status_service");
}

#[derive(Debug, Default)]
pub struct StatusService {}

#[tonic::async_trait]
impl StatusServiceRpc for StatusService {

}