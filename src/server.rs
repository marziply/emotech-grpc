use self::service::data_request::Input;
use self::service::service_server::{Service, ServiceServer};
use self::service::{DataRequest, DataResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::codegen::CompressionEncoding;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub const ADDRESS: &str = "http://0.0.0.0:50051";

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

#[cfg(test)]
mod tests {
  use super::service::service_client::ServiceClient;
  use super::service::service_server::ServiceServer;
  use super::{DataService, ADDRESS};
  use futures::future::BoxFuture;
  use futures::FutureExt;
  use std::time::Duration;
  use tokio::spawn;
  use tokio::sync::oneshot::channel;
  use tokio::time::sleep;
  use tonic::transport::{Channel, Server};

  async fn init() -> (
    impl FnOnce() -> BoxFuture<'static, ()>,
    ServiceClient<Channel>,
  ) {
    let (tx, rx) = channel::<()>();
    let addr = ADDRESS.parse().unwrap();
    let svc = ServiceServer::new(DataService::default());
    let handle = spawn(async move {
      Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, rx.map(drop))
        .await
        .unwrap();
    });
    let done = || {
      let run = async {
        tx.send(()).unwrap();
        handle.await.unwrap();
      };

      run.boxed()
    };

    sleep(Duration::from_millis(100)).await;

    let client = ServiceClient::connect("http://127.0.0.1:1338")
      .await
      .unwrap();

    (done, client)
  }

  #[tokio::test]
  async fn receive_string() {}

  #[tokio::test]
  async fn receive_number() {}

  #[tokio::test]
  async fn receive_file() {}
}
