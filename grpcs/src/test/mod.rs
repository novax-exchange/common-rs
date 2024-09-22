use tonic::service::Routes;
use tonic::{Request, Response, Status};

mod svc;

// grpc
use svc::{VoidMsg, RespMsg};
use svc::sample_svc_server::{SampleSvc, SampleSvcServer};

pub (crate) fn sample_routes() -> Routes {
    let rpc_svc = SvcImpl{};
    let svc = SampleSvcServer::new(rpc_svc);
    Routes::new(svc)
}

#[derive(Debug)]
pub (crate) struct SvcImpl {}

#[tonic::async_trait]
impl SampleSvc for SvcImpl {
    async fn call(&self, _request: Request<VoidMsg>) -> Result<Response<RespMsg>, Status> {
        Ok(
            Response::new(
                RespMsg {
                    code: 200, resp: "hello resp".to_string()
                }
            )
        )
    }
   
}