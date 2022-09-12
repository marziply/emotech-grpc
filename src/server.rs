use self::service::data_request::Input;
use self::service::service_server::{Service, ServiceServer};
use self::service::{DataRequest, DataResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::codegen::CompressionEncoding;
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
    req: Request<DataRequest>,
  ) -> Result<Response<DataResponse>, Status> {
    let DataRequest { input } = req.get_ref();
    let res = DataResponse { ok: true };

    match input.clone() {
      Some(data) => match data {
        Input::FileData(buf) => println!("Received file: {} bytes", buf.len()),
        _ => println!("Received data: {data:?}"),
      },
      None => println!("Error receiving data, corrupt input"),
    };

    Ok(Response::new(res))
  }
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
  let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
  let service = ServiceServer::new(DataService::default())
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

  println!("Server listening on {addr:#?}");

  Server::builder()
    .add_service(service)
    .serve(addr)
    .await?;

  Ok(())
}
