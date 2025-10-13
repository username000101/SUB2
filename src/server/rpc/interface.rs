use crate::server::rpc::decl::sub2_start_rpc_server;

pub async fn start_rpc_server() {
    sub2_start_rpc_server(5000).await;
}