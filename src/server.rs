use tonic::{transport::Server, Request, Response, Status};

use self::service::{service_server::Service, DataRequest, DataResponse};

pub mod service {
  tonic::include_proto!("emotech_service");
}

struct DataService;

#[tonic::async_trait]
impl Service for DataService {
  async fn receive_data(
    &self,
    request: Request<DataRequest>,
  ) -> Result<Response<DataResponse>, Status> {
    println!("Received request");

    let res = DataResponse { ok: true };

    Ok(Response::new(res))
  }
}

pub async fn start_server() {
  let _ = Server::builder();
}
