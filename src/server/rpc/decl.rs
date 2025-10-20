use mrpc_fixed::{RpcSender, Value};
use tracing::info;
use crate::sub2_get_version;
use crate::td::interface::CLIENT;

#[derive(Clone, Default)]
struct SUB2Service;

#[async_trait::async_trait]
impl mrpc_fixed::Connection for SUB2Service {
    async fn handle_request(&self, _client: RpcSender, method: &str, params: Vec<Value>) -> mrpc_fixed::Result<Value> {
        match method {
            "sub2.version" => Ok(sub2_get_version!()().into()),
            "sub2.ping" => Ok("PONG".into()),
            "sub2.echo" => {
                if params.is_empty() {
                    Ok("ERROR_TOO_FEW_ARGS".into())
                } else {
                    Ok(params[0].clone())
                }
            },
            "sub2.get_update" => {
                let mut lock = CLIENT.lock();
                let upd = lock.get_update();
                Ok(upd.into())
            },
            "sub2.send_request" => {
                if params.len() < 2 {
                    return Ok("ERROR_TOO_FEW_ARGS".into());
                }

                if !params[0].is_str() || !params[1].is_str() {
                    return Ok("ERROR_INVALID_ARGS".into());
                }

                let mut lock = CLIENT.lock();
                let val = serde_json::from_str::<serde_json::Value>(params[0].as_str().unwrap());
                if val.is_err() {
                    return Ok("ERROR_SERDEJSON_FAILED".into());
                }

                let mut response = lock.send(val.unwrap(), params[1].as_str().unwrap().to_string());
                Ok(response.get_raw().into())
            },
            &_ => Ok("ERROR_METHOD_NOT_FOUND".into()),
        }
    }
}
pub async fn sub2_start_rpc_server(port: u16) {
    let server = mrpc_fixed::Server::from_fn(SUB2Service::default).tcp(format!("127.0.0.1:{}", port.to_string()).as_str()).await.unwrap();
    info!("RPC server listening on port {}", port);
    server.run().await.expect("RPC server crashed");
}