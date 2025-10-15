use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};
use tracing::info;

pub mod sub2_api {
    tonic::include_proto!("sub2_api");
}

use sub2_api::{
    VersionResponse,
    SingleMessage,
    EmptyRequestArgs,
    TdApiRequest,
    sub2_server::{Sub2, Sub2Server}
};
use crate::sub2_get_version;

#[derive(Debug, Default)]
pub struct SUB2Service {}

#[tonic::async_trait]
impl Sub2 for SUB2Service {
    async fn version(&self, _req: Request<EmptyRequestArgs>) -> Result<Response<VersionResponse>, Status> {
        let result = VersionResponse {
            version: sub2_get_version!()()
        };
        Ok(Response::new(result))
    }

    async fn ping(&self, _: Request<EmptyRequestArgs>) -> Result<Response<SingleMessage>, Status> {
        let result = SingleMessage {
            message: "PONG".to_string()
        };
        Ok(Response::new(result))
    }

    async fn echo(&self, request: Request<SingleMessage>) -> Result<Response<SingleMessage>, Status> {
        let msg = request.into_inner();
        let result = SingleMessage {
            message: msg.message
        };
        Ok(Response::new(result))
    }

    async fn get_update(&self, _: Request<EmptyRequestArgs>) -> Result<Response<SingleMessage>, Status> {
        use crate::td::interface::CLIENT;

        let mut lock = CLIENT.lock();
        let result = SingleMessage {
            message: lock.get_update()
        };
        Ok(Response::new(result))
    }

    async fn send_request(&self, request: Request<TdApiRequest>) -> Result<Response<SingleMessage>, Status> {
        use crate::td::interface::CLIENT;
        
        let api_req = request.into_inner();
        let mut lock = CLIENT.lock();
        let req = serde_json::from_str::<serde_json::Value>(api_req.request.as_str());
        match req {
            Err(e) => {
                let result = SingleMessage { message: format!("INVALID_REQUEST: {}", e.to_string()) };
                Ok(Response::new(result))
            },
            Ok(val) => {
                let mut resp = lock.send(val, api_req.extra_field);
                let result = SingleMessage { message: resp.get_raw() };
                Ok(Response::new(result))
            }
        }
    }
}

pub async fn sub2_start_rpc_server(port: u16) {
    let addr = "127.0.0.1:5000".parse().unwrap();
    let svc = SUB2Service::default();
    
    info!("Starting RPC server on port {}", port);
    
    Server::builder()
        .add_service(Sub2Server::new(svc))
        .serve(addr).await.unwrap();
}