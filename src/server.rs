use self::service::service_server::{Service, ServiceServer};
use self::service::{DataRequest, DataResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod service {
  tonic::include_proto!("emotech_service");
}

#[derive(Debug, Default)]
struct DataService;

#[tonic::async_trait]
impl Service for DataService {
  async fn send_data(
    &self,
    _: Request<DataRequest>,
  ) -> Result<Response<DataResponse>, Status> {
    println!("Received request");

    let res = DataResponse { ok: true };

    Ok(Response::new(res))
  }
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
  let svc = DataService::default();
  let addr = SocketAddr::from(([0, 0, 0, 0], 50051));

  println!("Server listening on {addr:#?}");

  Server::builder()
    .add_service(ServiceServer::new(svc))
    .serve(addr)
    .await?;

  Ok(())
}
